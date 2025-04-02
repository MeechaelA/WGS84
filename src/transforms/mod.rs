pub mod transforms{
    use nalgebra::{RealField, ComplexField, Matrix3x1};
    use crate::constants::constants::{GM, g_e, b, k, e2, E, w, a};
    
    pub fn geocentric_to_ecef(latitude: f64, longitude: f64, altitude: f64)->Matrix3x1<f64>{
        let n = a / (1.0 - e2 * latitude.sin() * latitude.sin()).sqrt();
        let x = (n + altitude) * latitude.cos() * longitude.cos();
        let y = (n + altitude) * latitude.cos() * longitude.sin();
        let z = (n*(1.0-e2) + altitude)*latitude.sin();
        return Matrix3x1::new(x,y,z);
    }

    pub fn ecef_to_geocentric_ferrari(x: f64, y: f64, z: f64) -> Matrix3x1<f64>{
        let a2  = a*a;
        let b2  = b*b;
        let f   = (a-b)/a;
        let e_2 = 2.0*f-f*f;
        let ep  = (a2/b2 - 1.0).sqrt();
        let z2  = z*z;
        let r   = (x*x + y*y).sqrt();
        let r2  = r*r;
        let e2_2  = a2 - b2;
        let ff  = 54.0 * b2 * z2;
        let g   = r2 + (1.0 - e_2)*z2 - e_2*e2_2;
        let c   = e_2.powf(2.0) * ff * r2 / g.powf(3.0);
        let s   = (1.0 + c + (c.powf(2.0) + 2.0*c)).powf(1.0/3.0);
        let pp  = ff / ( 3.0*(s + 1.0/s + 1.0).powf(2.0) * g.powf(2.0));
        let q   = ( 1.0 + 2.0*e_2.powf(2.0) * pp ).sqrt();

        // NaNs are gross. -> This needs fixing. !TODO
        let mut r0_op = 1.0/2.0 * a2 * (1.0 + 1.0/q) - ( pp*(1.0-e_2)*z2 )/(q*(1.0+q)) - 1.0/2.0 * pp * r2;
        if (r0_op > 0.0) && (!r0_op.is_nan()){
            r0_op = r0_op;
        }
        else{
            r0_op = 0.0;
        }

        let r0  = -pp*e_2*r/(1.0+q) + (r0_op).sqrt();
        let u   = ( (r - e_2*r0).powf(2.0) + z2 ).sqrt();
        let v   = ( (r - e_2*r0).powf(2.0) + (1.0 - e_2)*z2 ).sqrt();
        let z0  = b2 * z / (a*v);


        let altitude = u * (1.0 - (b2 / (a * v)));
        let latitude = (z+ep*ep*z0).atan2(r);
        let longitude = y.atan2(x);
        return Matrix3x1::new(latitude, longitude, altitude);
    }

    pub fn ecef_to_geocentric(x: f64, y: f64, z: f64) -> Matrix3x1<f64> {
        // Compute intermediate quantities
        let eps = f64::EPSILON * 1.0e2;
        let rho2 = x * x + y * y;
        let mut dz = e2 * z;
        let mut N = 0.0;
    
        // Iterative refine coordinate estimate
        let mut iter = 0;
        while iter < 101 {
            let zdz = z + dz;
            let Nh = (rho2 + zdz * zdz).sqrt();
            let sinphi = zdz / Nh;
            N = a / (1.0 - e2 * sinphi * sinphi).sqrt();
            let dz_new = N * e2 * sinphi;
    
            // Check convergence requirement
            if (dz - dz_new).abs() < eps {
                break;
            }
    
            dz = dz_new;
            iter += 1;
        }
    
        if iter == 100 {
            panic!("Reached maximum number of iterations.");
        }
    
        let zdz = z + dz;
        let lon = y.atan2(x);
        let lat = zdz.atan2(rho2.sqrt());
        let alt = (rho2 + zdz * zdz).sqrt() - N;
    
        return Matrix3x1::new(lat, lon, alt)
    }
}