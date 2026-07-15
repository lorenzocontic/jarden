use crate::models::Screenshot;

pub fn load() -> Vec<Screenshot> {

    vec![

        Screenshot{
            name:String::from("screenshot_0512.png"),
            month:String::from("2026-05")
        },

        Screenshot{
            name:String::from("screenshot_0528.png"),
            month:String::from("2026-05")
        },

        Screenshot{
            name:String::from("screenshot_0602.png"),
            month:String::from("2026-06")
        },

        Screenshot{
            name:String::from("screenshot_0705.png"),
            month:String::from("2026-07")
        },

        Screenshot{
            name:String::from("screenshot_0712.png"),
            month:String::from("2026-07")
        }

    ]

}
