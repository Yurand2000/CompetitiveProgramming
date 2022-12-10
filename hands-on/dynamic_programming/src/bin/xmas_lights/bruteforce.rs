use crate::*;

fn __bruteforce_solve(lights: Vec<XmasLight>) -> (i32, i32, i32)
{
    let (mut red, mut rw_combs, mut rwg_combs) = (0, 0, 0);

    let mut explicit_comb_of_lights = lights.clone();
    let mut unknowns_lights = 0;
    for light in explicit_comb_of_lights.iter_mut() {
        if *light == XmasLight::Unknown {
            *light = XmasLight::Red;
            unknowns_lights += 1;
        } 
    }

    let combinations = 3u32.pow(unknowns_lights);
    for i in 0..combinations {
        let mut modulus = i;
        let mut k = unknowns_lights as i32 - 1;
        for (index, light) in lights.iter().enumerate() {
            if *light == XmasLight::Unknown {
                let color = modulus / 3u32.pow(k as u32);
                modulus = modulus % 3u32.pow(k as u32);
                k -= 1;
    
                match color {
                    0 => explicit_comb_of_lights[index] = XmasLight::Red,
                    1 => explicit_comb_of_lights[index] = XmasLight::White,
                    2 => explicit_comb_of_lights[index] = XmasLight::Green, 
                    v => panic!("{v}"),
                }
            }
        }

        let (new_red, new_white, new_green) = solve_simple(&explicit_comb_of_lights);
        red += new_red;
        rw_combs += new_white;
        rwg_combs += new_green;
    }

    (red, rw_combs, rwg_combs)
}