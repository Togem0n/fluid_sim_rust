const SIZE: usize = 100;

pub fn IX(x: usize, y: usize) -> usize {
    x + y * SIZE
}

pub struct fluid_cube{
    pub size: usize,
    pub dt: f64,
    pub diff: f64,
    pub visc: f64,

    pub s: [f64; SIZE * SIZE],
    pub density: [f64; SIZE * SIZE], 

    pub vx: [f64; SIZE * SIZE],
    pub vy: [f64; SIZE * SIZE],

    pub vx0: [f64; SIZE * SIZE],
    pub vy0: [f64; SIZE * SIZE],
}

impl fluid_cube {
   pub fn fluid_cube_create(size: usize, diff: f64, visc: f64, dt: f64) -> Self{
        fluid_cube{
            size: SIZE,
            diff: diff,
            visc: visc,
            dt: dt,
            s: [0.0; SIZE * SIZE],
            density: [0.0; SIZE * SIZE],
            vx: [0.0; SIZE * SIZE],
            vy: [0.0; SIZE * SIZE],
            vx0: [0.0; SIZE * SIZE],
            vy0: [0.0; SIZE * SIZE],
        }
   } 

   pub fn add_density(&mut self, x: usize, y: usize, amount: f64) {
        self.density[IX(x, y)] += amount;
   }

   pub fn add_velocity(&mut self, x: usize, y: usize, amount_x: f64, amount_y: f64){
        self.vx[IX(x, y)] += amount_x;
        self.vy[IX(x, y)] += amount_y;
   }

   // physics

   pub fn step(&mut self){

        self.diffuse(1, self.vx0, self.vx, self.visc, self.dt, 4);
        self.diffuse(2, self.vy0, self.vy, self.visc, self.dt, 4);

        self.project(self.vx0, self.vy0, self.vx, self.vy, 4);
        
        self.advert(1, self.vx, self.vx0, self.vx0, self.vy0, self.dt);
        self.advert(2, self.vy, self.vy0, self.vx0, self.vy0, self.dt);

        self.project(self.vx, self.vy, self.vx0, self.vy0, 4);

        self.diffuse(0, self.s, self.density, self.diff, self.dt, 4);
        self.advert(0, self.density, self.s, self.vx, self.vy, self.dt);
   }

   pub fn set_bnd(&mut self, b: usize, mut x: [f64; SIZE * SIZE], size: usize){

        let n = SIZE;

        for i in 1..n-1 {
            x[IX(i, 0)] = match b{
                2 => -x[IX(i, 1)],
                _ => x[IX(i, 1)],
            };

            x[IX(i, n-1)] = match b {
               2 => -x[IX(i, n-2)],
               _ => x[IX(i, n-2)],
            };
        }

        for j in 1..n-1 {
            x[IX(0, j)] = match b {
                1 => -x[IX(1, j)],
                _ => x[IX(1, j)],
            };

            x[IX(n-1, j)] = match b {
                1 => -x[IX(n-2, j)],
                _ => x[IX(n-2, j)],
            };
        }
        
        x[IX(0, 0)] = 0.33 * (x[IX(1, 0)]
                                        + x[IX(0, 1)] 
                                        + x[IX(0, 0)]);

        x[IX(0, n-1)] = 0.33 * (x[IX(1, n-1)]
                                        + x[IX(0, n-2)] 
                                        + x[IX(0, n-1)]);

        x[IX(n-1, 0)] = 0.33 * (x[IX(n-2, 0)]
                                        + x[IX(n-1, 1)] 
                                        + x[IX(n-1, 0)]);

        x[IX(n-1, n-1)] = 0.33 * (x[IX(n-2, n-1)]
                                        + x[IX(n-1, n-2)] 
                                        + x[IX(n-1, n-1)]);
   }

   pub fn lin_solve(&mut self, b: usize, mut x: [f64; SIZE * SIZE],
                     mut x0: [f64; SIZE * SIZE], a: f64, c: f64, iter: usize){
        let c_recip: f64 = 1.0 / c;
        for k in 0..iter {
            for j in 1..SIZE-1 {
                for i in 1..SIZE-1 {
                    x[IX(i, j)] = (x0[IX(i, j)] + a *
                            ( x[IX(i + 1, j)]
                            + x[IX(i - 1, j)]
                            + x[IX(i, j + 1)]
                            + x[IX(i, j - 1)]
                            + x[IX(i, j)]
                            + x[IX(i, j)]
                                )) * c_recip;
                }
            }
            self.set_bnd(b, x, SIZE);
        }
   }

   pub fn diffuse(&mut self, b: usize, mut x: [f64; SIZE * SIZE], 
                            mut x0: [f64; SIZE * SIZE], 
                            diff: f64, dt: f64, iter: usize){
        let SIZE_F = SIZE as f64;
        let a: f64 = dt * diff * (SIZE_F - 2.0) * (SIZE_F - 2.0);
        self.lin_solve(b, x, x0, a, 1.0 + 6.0 * a, iter)
   }

   pub fn project(&mut self, mut vel_x: [f64; SIZE * SIZE],
                            mut vel_y: [f64; SIZE * SIZE],
                            mut p: [f64; SIZE * SIZE],
                            mut div: [f64; SIZE * SIZE],
                            iter: usize){
        let n = SIZE;
        let SIZE_F = SIZE as f64;
        for j in 1..n-1 {
            for i in 1..n-1 {
                div[IX(i, j)] = -0.5 * (
                    vel_x[IX(i+1, j)] +
                   -vel_x[IX(i-1, j)] +
                    vel_y[IX(i, j+1)] +
                   -vel_y[IX(i, j-1)]
                ) / SIZE_F;
                p[IX(i, j)] = 0.0;
            }
        }

        self.set_bnd(0, div, SIZE);
        self.set_bnd(0, p, SIZE);
        self.lin_solve(0, p, div, 1.0, 6.0, iter);
        
        for j in 1..SIZE-1 {
            for i in 1..SIZE-1 {
                vel_x[IX(i, j)] -= 0.5 * (p[IX(i+1, j)] - p[IX(i-1, j)]) * SIZE_F;
                vel_x[IX(i, j)] -= 0.5 * (p[IX(i, j+1)] - p[IX(i, j-1)]) * SIZE_F;
            }
        }

        self.set_bnd(1, vel_x, SIZE);
        self.set_bnd(2, vel_y, SIZE);
    }

    pub fn advert(&mut self, b: usize, mut d: [f64; SIZE * SIZE],
        mut d0: [f64; SIZE * SIZE], mut vel_x: [f64; SIZE * SIZE],
        mut vel_y: [f64; SIZE * SIZE], dt: f64){

        let SIZE_F: f64 = SIZE as f64;

        let mut i0: f64; 
        let mut i1: f64;
        let mut j0: f64; 
        let mut j1: f64;

        let mut dtx: f64 = dt * (SIZE_F - 2.0);
        let mut dty: f64 = dt * (SIZE_F - 2.0);

        let mut s0: f64; 
        let mut s1: f64;
        let mut t0: f64; 
        let mut t1: f64;

        let mut tmp1: f64; 
        let mut tmp2: f64;
        let mut x: f64; 
        let mut y: f64;

        let mut i: usize = 1;
        let mut j: usize = 1;
        let mut ifloat: f64 = 1.0;
        let mut jfloat: f64 = 1.0;
        
        for j in 1..SIZE-1 {
            for i in 1..SIZE-1 {
                tmp1 = dtx * (vel_x[IX(i, j)]);
                tmp2 = dty * (vel_y[IX(i, j)]);
                x = ifloat - tmp1;
                y = jfloat - tmp2;

                if x < 0.5 { x = 0.5; }
                if x > (SIZE_F + 0.5) { x = SIZE_F + 0.5; }
                i0 = x.floor();
                i1 = i0 + 1.0;

                if y < 0.5 { y = 0.5; }
                if y > (SIZE_F + 0.5 ) { y = SIZE_F + 0.5; }
                j0 = y.floor();
                j1 = j0 + 1.0;

                s1 = x - i0;
                s0 = 1.0 - s1;

                t1 = y - j0;
                t0 = 1.0 - t1;

                let i0i = i0 as usize;
                let i1i = i1 as usize;
                let j0i = j0 as usize;
                let j1i = j1 as usize;

                d[IX(i, j)] = s0 * (t0 * d0[IX(i0i, j0i)] + t1 * d0[IX(i0i, j1i)])
                                +  s1 * (t0 * d0[IX(i1i, j0i)] + t1 * d0[IX(i1i, j1i)]);
                ifloat += 1.0;
            }
            jfloat += 1.0;
        }
        self.set_bnd(b, d, SIZE)
    }

}