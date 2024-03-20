    #[derive(PartialEq)]
    pub enum Structure {
        Glider,
        Lwss,
        Mwss,
        Hwss,
        Rpent,
        Diehard,
        Acorn,
        GosperGun,
        Infinite1,
    }

    impl Structure {
        pub fn to_usize(&self) -> usize {
            match self {
                Structure::Glider => 0,
                Structure::Lwss => 1,
                Structure::Mwss => 2,
                Structure::Hwss => 3,
                Structure::Rpent => 4,
                Structure::Diehard => 5,
                Structure::Acorn => 6,
                Structure::GosperGun => 7,
                Structure::Infinite1 => 8
            }
        }

        pub fn from_usize(value: usize) -> Option<Structure> {
            match value {
                0 => Some(Structure::Glider),
                1 => Some(Structure::Lwss),
                2 => Some(Structure::Mwss),
                3 => Some(Structure::Hwss),
                4 => Some(Structure::Rpent),
                5 => Some(Structure::Diehard),
                6 => Some(Structure::Acorn),
                7 => Some(Structure::GosperGun),
                8 => Some(Structure::Infinite1),
                _ => None
            }
        }
    }

    pub fn get_structure_vec(strctr: Structure) -> Vec<Vec<u8>> {
        match strctr {
            Structure::Glider => {
                vec![
                    vec![0, 0, 1],
                    vec![1, 0, 1],
                    vec![0, 1, 1]
                ]
            },
            Structure::Lwss => {
                vec![
                    vec![0, 1, 1, 1, 1],
                    vec![1, 0, 0, 0, 1],
                    vec![0, 0, 0, 0, 1],
                    vec![1, 0, 0, 1, 0]
                ]
            },
            Structure::Mwss => {
                vec![
                    vec![0, 0, 1, 0, 0, 0],
                    vec![1, 0, 0, 0, 1, 0],
                    vec![0, 0, 0, 0, 0, 1],
                    vec![1, 0, 0, 0, 0, 1],
                    vec![0, 1, 1, 1, 1, 1]
                ]
            },
            Structure::Hwss => {
                vec![
                    vec![0, 0, 1, 1, 0, 0, 0],
                    vec![1, 0, 0, 0, 0, 1, 0],
                    vec![0, 0, 0, 0, 0, 0, 1],
                    vec![1, 0, 0, 0, 0, 0, 1],
                    vec![0, 1, 1, 1, 1, 1, 1]
                ]
            },
            Structure::Rpent => {
                vec![
                    vec![0, 1, 1],
                    vec![1, 1, 0],
                    vec![0, 1, 0]
                ]
            },
            Structure::Diehard => {
                vec![
                    vec![0, 0, 0, 0, 0, 0, 1, 0],
                    vec![1, 1, 0, 0, 0, 0, 0, 0],
                    vec![0, 1, 0, 0, 0, 1, 1, 1]
                ]
            },
            Structure::Acorn => {
                vec![
                    vec![0, 1, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 1, 0, 0, 0],
                    vec![1, 1, 0, 0, 1, 1, 1]
                ]
            },
            Structure::GosperGun => {
                vec![
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
                    vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ]
            },
            Structure::Infinite1 => {
                vec![
                    vec![1, 1, 1, 0, 1],
                    vec![1, 0, 0, 0, 0],
                    vec![0, 0, 0, 1, 1],
                    vec![0, 1, 1, 0, 1],
                    vec![1, 0, 1, 0, 1]
                ]
            }
            _ => {
                vec![]
            }
        }
    }
