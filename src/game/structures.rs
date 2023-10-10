pub mod structuress {
    #[derive(PartialEq)]
    pub enum Structure {
        Glider,
        LWSS,
        MWSS,
        HWSS,
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
                Structure::LWSS => 1,
                Structure::MWSS => 2,
                Structure::HWSS => 3,
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
                1 => Some(Structure::LWSS),
                2 => Some(Structure::MWSS),
                3 => Some(Structure::HWSS),
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
            Structure::LWSS => {
                vec![
                    vec![0, 1, 1, 1, 1],
                    vec![1, 0, 0, 0, 1],
                    vec![0, 0, 0, 0, 1],
                    vec![1, 0, 0, 1, 0]
                ]
            },
            Structure::MWSS => {
                vec![
                    vec![0, 0, 1, 0, 0, 0],
                    vec![1, 0, 0, 0, 1, 0],
                    vec![0, 0, 0, 0, 0, 1],
                    vec![1, 0, 0, 0, 0, 1],
                    vec![0, 1, 1, 1, 1, 1]
                ]
            },
            Structure::HWSS => {
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
}