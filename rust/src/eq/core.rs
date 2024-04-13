use fundsp::hacker::*;
use std::{collections::HashMap, vec};

//struct Equalizer<T>(T);

struct Profile {
    name: String,
    size: Int,
    equalizer: An<>,
    bands: Vec<(f64,f64,f64)>,
}

impl Profile {
    fn set_profile_bands(self: &Self, ) {
        vec
        seld.bands
    }
}

// impl Profile {
//     fn
// }

pub fn calc_bands(x: f64) -> Vec<(f64,f64,f64)>{

}

pub fn init_profile() -> Profile<String, An<>> {
    let mut new_name = String::from("New Profile");

    let mut new_equalizer = pipe::<U12, _, _>(|i| bell_hz(1000.0 + 1000.0 * i as f64, 1.0, db_amp(0.0)));

    let mut new_bands = vec![];
    for i in 12 {
        new_bands.append((0,0,0));
    }

    let mut new_profile = Profile {
        name: String::from("new_profile"),
        size: 12,
        equalizer: new_equalizer,
        bands: new_bands
    };
    //let mut test = An
    new_profile
}

// pub fn init_profile() -> Profile<String, An<>> {
//     let mut new_name = "New Profile";
//     // Default utilities - feel free to customize
//     let mut new_equalizer = pipe::<U12, _, _>(|i| bell());
//     let mut node = An<0>
    
//     let mut new_profile = Equalizer {
//         name: new_name,
//         size: 12,
//         equalizer: new_node,
//         bands: 
//     };
//     //let mut test = An
//     new_profile
// }


pub fn set_gain(input_profile: &Profile, band: Int, gain: f64) {
    input_profile.equalizer.node_mut(band).set_gain(db_amp(gain));
}

pub fn set_center(input_profile: &Profile, band: Int, center: f64) {
    input_profile.equalizer.node_mut(band).set_center(center);
}

pub fn set_q(input_profile: &Profile, band: Int, q: f64) {
    input_profile.equalizer.node_mut(band).set_q(q);
}

pub fn add_band(input_profile: &Profile){
    if (input_profile.size < 50) {
        let size_mappings = get_band_mapping();
        let mut new_mapping = size_mappings.entry(input_profile.size+1);

        let mut new_equalizer = pipe::<new_mapping, _, _>(|i| bell_hz(1000.0 + 1000.0 * i as f64, 1.0, db_amp(0.0)));

        let mut new_profile = Profile {
            name: input_profile.name,
            size: input_profile.size+1,
            equalizer: new_equalizer,
            bands: input_profile.bands.append((0,0,0)),
        };

        for i in input_profile.size {
            set_gain(&new_profile, band, input_profile.bands[i].0);
            set_center(&new_profile, band, input_profile.bands[i].1);
            set_q(&new_profile, band, input_profile.bands[i].2);
        }
    }
}


pub fn remove_band(input_profile: &Profile, band: Int){
    if (input_profile.size > 1) {
        let size_mappings = get_band_mapping();
        let mut new_mapping = size_mappings.entry(input_profile.size-1);

        let mut new_equalizer = pipe::<new_mapping, _, _>(|i| bell_hz(1000.0 + 1000.0 * i as f64, 1.0, db_amp(0.0)));

        let mut new_bands = vec![];
        for i in input_profile.bands.len() {
            if (i != band){
                new_bands.append(input_profile.bands[i]);
            }
        }

        let mut new_profile = Profile {
            name: input_profile.name,
            size: input_profile.size-1,
            equalizer: new_equalizer,
            bands: new_bands
        };

        for i in input_profile.size {
            if (i != band){
                set_gain(&new_profile, band, input_profile.bands[i].0);
                set_center(&new_profile, band, input_profile.bands[i].1);
                set_q(&new_profile, band, input_profile.bands[i].2);
            }  
        }
    }
}

pub fn get_band_mapping() -> HashMap<Int, T> {
    let band_mapping = HashMap::from([
        (1, U1),
        (2, U2),
        (3, U3),
        (4, U4),
        (5, U5),
        (6, U6),
        (7, U7),
        (8, U8),
        (9, U9),
        (10, U10),
        (11, U11),
        (12, U12),
        (13, U13),
        (14, U14),
        (15, U15),
        (16, U16),
        (17, U17),
        (18, U18),
        (19, U19),
        (20, U20),
        (21, U21),
        (22, U22),
        (23, U23),
        (24, U24),
        (25, U25),
        (26, U26),
        (27, U27),
        (28, U28),
        (29, U29),
        (30, U30),
        (31, U31),
        (32, U32),
        (33, U33),
        (34, U34),
        (35, U35),
        (36, U36),
        (37, U37),
        (38, U38),
        (39, U39),
        (40, U40),
        (41, U41),
        (42, U42),
        (43, U43),
        (44, U44),
        (45, U45),
        (46, U46),
        (47, U47),
        (48, U48),
        (49, U49),
        (50, U50),
    ]);
    band_mapping
}

