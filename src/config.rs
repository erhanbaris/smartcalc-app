use std::cell::RefCell;

use lazy_static::*;

#[derive(Default, PartialEq)]
#[derive(Debug, Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Timezone {
    pub offset: f32,
    pub name: String,
    pub isdst: bool
}

impl ToString for Timezone {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
}

impl Timezone {
    pub fn abbr(&self) -> String {
        let trunch_part = self.offset.trunc() as i32;
        let fract_part = (self.offset.fract() * 60.0) as i32;

        let trunch_part = if trunch_part == 0 { String::new() }
            else if trunch_part > 0 { format!("+{}", trunch_part) }
            else { format!("{}", trunch_part) };

        let fract_part = if fract_part == 0 { String::new() }
            else { format!(":{:02}", fract_part) };

        format!("GMT{}{}", trunch_part, fract_part)
    }
}

thread_local!(pub static CURRENT_TIMEZONE: RefCell<Timezone> = RefCell::new(Timezone {
    offset: 0.0,
    name: "UTC".to_string(),
    isdst: false
}));

pub fn update_current_timezone(timezone: &Timezone) {
    CURRENT_TIMEZONE.with(|current_timezone| {
        *current_timezone.borrow_mut() = timezone.clone();
    });
}

pub fn get_current_timezone() -> Timezone {
    CURRENT_TIMEZONE.with(|current_timezone| {
        current_timezone.borrow().clone()
    })
}

lazy_static! {
    pub static ref UTC_TIMEZONE: Timezone = {
        let m = Timezone {
            offset: 0.0,
            name: "UTC".to_string(),
            isdst: false
        };
        m
    };

    pub static ref TIMEZONE_LIST: Vec<Timezone> = {
        let m = vec![Timezone {
            offset: 0.0,
            name: "Africa/Abidjan".to_string(),
            isdst: false
        }, Timezone {
            offset: 0.0,
            name: "Africa/Accra".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Africa/Addis_Ababa".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Africa/Algiers".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Africa/Asmera".to_string(),
            isdst: false
        }, Timezone {
            offset: 0.0,
            name: "Africa/Bamako".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Africa/Bangui".to_string(),
            isdst: false
        }, Timezone {
            offset: 0.0,
            name: "Africa/Banjul".to_string(),
            isdst: false
        }, Timezone {
            offset: 0.0,
            name: "Africa/Bissau".to_string(),
            isdst: false
        }, Timezone {
            offset: 2.0,
            name: "Africa/Blantyre".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Africa/Brazzaville".to_string(),
            isdst: false
        }, Timezone {
            offset: 2.0,
            name: "Africa/Bujumbura".to_string(),
            isdst: false
        }, Timezone {
            offset: 2.0,
            name: "Africa/Cairo".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Africa/Casablanca".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Africa/Ceuta".to_string(),
            isdst: true
        }, Timezone {
            offset: 0.0,
            name: "Africa/Conakry".to_string(),
            isdst: false
        }, Timezone {
            offset: 0.0,
            name: "Africa/Dakar".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Africa/Dar_es_Salaam".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Africa/Djibouti".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Africa/Douala".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Africa/El_Aaiun".to_string(),
            isdst: true
        }, Timezone {
            offset: 0.0,
            name: "Africa/Freetown".to_string(),
            isdst: false
        }, Timezone {
            offset: 2.0,
            name: "Africa/Gaborone".to_string(),
            isdst: false
        }, Timezone {
            offset: 2.0,
            name: "Africa/Harare".to_string(),
            isdst: false
        }, Timezone {
            offset: 2.0,
            name: "Africa/Johannesburg".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Africa/Juba".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Africa/Kampala".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Africa/Khartoum".to_string(),
            isdst: false
        }, Timezone {
            offset: 2.0,
            name: "Africa/Kigali".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Africa/Kinshasa".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Africa/Lagos".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Africa/Libreville".to_string(),
            isdst: false
        }, Timezone {
            offset: 0.0,
            name: "Africa/Lome".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Africa/Luanda".to_string(),
            isdst: false
        }, Timezone {
            offset: 2.0,
            name: "Africa/Lubumbashi".to_string(),
            isdst: false
        }, Timezone {
            offset: 2.0,
            name: "Africa/Lusaka".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Africa/Malabo".to_string(),
            isdst: false
        }, Timezone {
            offset: 2.0,
            name: "Africa/Maputo".to_string(),
            isdst: false
        }, Timezone {
            offset: 2.0,
            name: "Africa/Maseru".to_string(),
            isdst: false
        }, Timezone {
            offset: 2.0,
            name: "Africa/Mbabane".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Africa/Mogadishu".to_string(),
            isdst: false
        }, Timezone {
            offset: 0.0,
            name: "Africa/Monrovia".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Africa/Nairobi".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Africa/Ndjamena".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Africa/Niamey".to_string(),
            isdst: false
        }, Timezone {
            offset: 0.0,
            name: "Africa/Nouakchott".to_string(),
            isdst: false
        }, Timezone {
            offset: 0.0,
            name: "Africa/Ouagadougou".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Africa/Porto-Novo".to_string(),
            isdst: false
        }, Timezone {
            offset: 0.0,
            name: "Africa/Sao_Tome".to_string(),
            isdst: false
        }, Timezone {
            offset: 2.0,
            name: "Africa/Tripoli".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Africa/Tunis".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Africa/Windhoek".to_string(),
            isdst: false
        }, Timezone {
            offset: -8.0,
            name: "America/Anchorage".to_string(),
            isdst: true
        }, Timezone {
            offset: -4.0,
            name: "America/Anguilla".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Antigua".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "America/Araguaina".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "America/Argentina/La_Rioja".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "America/Argentina/Rio_Gallegos".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "America/Argentina/Salta".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "America/Argentina/San_Juan".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "America/Argentina/San_Luis".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "America/Argentina/Tucuman".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "America/Argentina/Ushuaia".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Aruba".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Asuncion".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "America/Bahia".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "America/Bahia_Banderas".to_string(),
            isdst: true
        }, Timezone {
            offset: -4.0,
            name: "America/Barbados".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "America/Belem".to_string(),
            isdst: false
        }, Timezone {
            offset: -6.0,
            name: "America/Belize".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Blanc-Sablon".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Boa_Vista".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "America/Bogota".to_string(),
            isdst: false
        }, Timezone {
            offset: -6.0,
            name: "America/Boise".to_string(),
            isdst: true
        }, Timezone {
            offset: -3.0,
            name: "America/Buenos_Aires".to_string(),
            isdst: false
        }, Timezone {
            offset: -6.0,
            name: "America/Cambridge_Bay".to_string(),
            isdst: true
        }, Timezone {
            offset: -4.0,
            name: "America/Campo_Grande".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "America/Cancun".to_string(),
            isdst: true
        }, Timezone {
            offset: -4.5,
            name: "America/Caracas".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "America/Catamarca".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "America/Cayenne".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "America/Cayman".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "America/Chicago".to_string(),
            isdst: true
        }, Timezone {
            offset: -6.0,
            name: "America/Chihuahua".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/Coral_Harbour".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "America/Cordoba".to_string(),
            isdst: false
        }, Timezone {
            offset: -6.0,
            name: "America/Costa_Rica".to_string(),
            isdst: false
        }, Timezone {
            offset: -7.0,
            name: "America/Creston".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Cuiaba".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Curacao".to_string(),
            isdst: false
        }, Timezone {
            offset: 0.0,
            name: "America/Danmarkshavn".to_string(),
            isdst: false
        }, Timezone {
            offset: -7.0,
            name: "America/Dawson".to_string(),
            isdst: false
        }, Timezone {
            offset: -7.0,
            name: "America/Dawson_Creek".to_string(),
            isdst: false
        }, Timezone {
            offset: -6.0,
            name: "America/Denver".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/Detroit".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Detroit".to_string(),
            isdst: true
        }, Timezone {
            offset: -4.0,
            name: "America/Dominica".to_string(),
            isdst: false
        }, Timezone {
            offset: -6.0,
            name: "America/Edmonton".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/Eirunepe".to_string(),
            isdst: false
        }, Timezone {
            offset: -6.0,
            name: "America/El_Salvador".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "America/Fortaleza".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "America/Glace_Bay".to_string(),
            isdst: true
        }, Timezone {
            offset: -3.0,
            name: "America/Godthab".to_string(),
            isdst: true
        }, Timezone {
            offset: -3.0,
            name: "America/Goose_Bay".to_string(),
            isdst: true
        }, Timezone {
            offset: -4.0,
            name: "America/Grand_Turk".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Grenada".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Guadeloupe".to_string(),
            isdst: false
        }, Timezone {
            offset: -6.0,
            name: "America/Guatemala".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "America/Guayaquil".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Guyana".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "America/Halifax".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/Havana".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Havana".to_string(),
            isdst: true
        }, Timezone {
            offset: -7.0,
            name: "America/Hermosillo".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "America/Indiana/Knox".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/Indiana/Marengo".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "America/Indiana/Petersburg".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Indiana/Petersburg".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/Indiana/Tell_City".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/Indiana/Vevay".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "America/Indiana/Vincennes".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Indiana/Vincennes".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/Indiana/Winamac".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Indiana/Winamac".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/Indianapolis".to_string(),
            isdst: false
        }, Timezone {
            offset: -6.0,
            name: "America/Inuvik".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/Iqaluit".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Iqaluit".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/Jamaica".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "America/Jujuy".to_string(),
            isdst: false
        }, Timezone {
            offset: -8.0,
            name: "America/Juneau".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/Kentucky/Monticello".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Kentucky/Monticello".to_string(),
            isdst: true
        }, Timezone {
            offset: -4.0,
            name: "America/Kralendijk".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/La_Paz".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "America/Lima".to_string(),
            isdst: false
        }, Timezone {
            offset: -7.0,
            name: "America/Los_Angeles".to_string(),
            isdst: true
        }, Timezone {
            offset: -8.0,
            name: "America/Los_Angeles".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "America/Louisville".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Louisville".to_string(),
            isdst: true
        }, Timezone {
            offset: -4.0,
            name: "America/Lower_Princes".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "America/Maceio".to_string(),
            isdst: false
        }, Timezone {
            offset: -6.0,
            name: "America/Managua".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Manaus".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Marigot".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Martinique".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "America/Matamoros".to_string(),
            isdst: true
        }, Timezone {
            offset: -6.0,
            name: "America/Mazatlan".to_string(),
            isdst: true
        }, Timezone {
            offset: -3.0,
            name: "America/Mendoza".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "America/Menominee".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/Merida".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/Mexico_City".to_string(),
            isdst: true
        }, Timezone {
            offset: -3.0,
            name: "America/Moncton".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/Monterrey".to_string(),
            isdst: true
        }, Timezone {
            offset: -3.0,
            name: "America/Montevideo".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "America/Montreal".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Montreal".to_string(),
            isdst: true
        }, Timezone {
            offset: -4.0,
            name: "America/Montserrat".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "America/Nassau".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Nassau".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/New_York".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/New_York".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/Nipigon".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Nipigon".to_string(),
            isdst: true
        }, Timezone {
            offset: -8.0,
            name: "America/Nome".to_string(),
            isdst: true
        }, Timezone {
            offset: -2.0,
            name: "America/Noronha".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "America/North_Dakota/Beulah".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/North_Dakota/Center".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/North_Dakota/New_Salem".to_string(),
            isdst: true
        }, Timezone {
            offset: -6.0,
            name: "America/Ojinaga".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/Panama".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "America/Pangnirtung".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Pangnirtung".to_string(),
            isdst: true
        }, Timezone {
            offset: -3.0,
            name: "America/Paramaribo".to_string(),
            isdst: false
        }, Timezone {
            offset: -7.0,
            name: "America/Phoenix".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Port_of_Spain".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "America/Port-au-Prince".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Port-au-Prince".to_string(),
            isdst: true
        }, Timezone {
            offset: -4.0,
            name: "America/Porto_Velho".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Puerto_Rico".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "America/Rainy_River".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/Rankin_Inlet".to_string(),
            isdst: true
        }, Timezone {
            offset: -3.0,
            name: "America/Recife".to_string(),
            isdst: false
        }, Timezone {
            offset: -6.0,
            name: "America/Regina".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "America/Resolute".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/Rio_Branco".to_string(),
            isdst: false
        }, Timezone {
            offset: -7.0,
            name: "America/Santa_Isabel".to_string(),
            isdst: true
        }, Timezone {
            offset: -3.0,
            name: "America/Santarem".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Santiago".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Santo_Domingo".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "America/Sao_Paulo".to_string(),
            isdst: false
        }, Timezone {
            offset: 0.0,
            name: "America/Scoresbysund".to_string(),
            isdst: true
        }, Timezone {
            offset: -8.0,
            name: "America/Sitka".to_string(),
            isdst: true
        }, Timezone {
            offset: -4.0,
            name: "America/St_Barthelemy".to_string(),
            isdst: false
        }, Timezone {
            offset: -2.5,
            name: "America/St_Johns".to_string(),
            isdst: true
        }, Timezone {
            offset: -4.0,
            name: "America/St_Kitts".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/St_Lucia".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/St_Thomas".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/St_Vincent".to_string(),
            isdst: false
        }, Timezone {
            offset: -6.0,
            name: "America/Swift_Current".to_string(),
            isdst: false
        }, Timezone {
            offset: -6.0,
            name: "America/Tegucigalpa".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "America/Thule".to_string(),
            isdst: true
        }, Timezone {
            offset: -5.0,
            name: "America/Thunder_Bay".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Thunder_Bay".to_string(),
            isdst: true
        }, Timezone {
            offset: -7.0,
            name: "America/Tijuana".to_string(),
            isdst: true
        }, Timezone {
            offset: -8.0,
            name: "America/Tijuana".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "America/Toronto".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "America/Toronto".to_string(),
            isdst: true
        }, Timezone {
            offset: -4.0,
            name: "America/Tortola".to_string(),
            isdst: false
        }, Timezone {
            offset: -7.0,
            name: "America/Vancouver".to_string(),
            isdst: true
        }, Timezone {
            offset: -8.0,
            name: "America/Vancouver".to_string(),
            isdst: false
        }, Timezone {
            offset: -7.0,
            name: "America/Whitehorse".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "America/Winnipeg".to_string(),
            isdst: true
        }, Timezone {
            offset: -8.0,
            name: "America/Yakutat".to_string(),
            isdst: true
        }, Timezone {
            offset: -6.0,
            name: "America/Yellowknife".to_string(),
            isdst: true
        }, Timezone {
            offset: 8.0,
            name: "Antarctica/Casey".to_string(),
            isdst: false
        }, Timezone {
            offset: 7.0,
            name: "Antarctica/Davis".to_string(),
            isdst: false
        }, Timezone {
            offset: 10.0,
            name: "Antarctica/DumontDUrville".to_string(),
            isdst: false
        }, Timezone {
            offset: 11.0,
            name: "Antarctica/Macquarie".to_string(),
            isdst: false
        }, Timezone {
            offset: 5.0,
            name: "Antarctica/Mawson".to_string(),
            isdst: false
        }, Timezone {
            offset: 12.0,
            name: "Antarctica/McMurdo".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "Antarctica/Palmer".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "Antarctica/Rothera".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Antarctica/Syowa".to_string(),
            isdst: false
        }, Timezone {
            offset: 6.0,
            name: "Antarctica/Vostok".to_string(),
            isdst: false
        }, Timezone {
            offset: 2.0,
            name: "Arctic/Longyearbyen".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Asia/Aden".to_string(),
            isdst: false
        }, Timezone {
            offset: 6.0,
            name: "Asia/Almaty".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Asia/Amman".to_string(),
            isdst: false
        }, Timezone {
            offset: 12.0,
            name: "Asia/Anadyr".to_string(),
            isdst: false
        }, Timezone {
            offset: 5.0,
            name: "Asia/Aqtau".to_string(),
            isdst: false
        }, Timezone {
            offset: 5.0,
            name: "Asia/Aqtobe".to_string(),
            isdst: false
        }, Timezone {
            offset: 5.0,
            name: "Asia/Ashgabat".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Asia/Baghdad".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Asia/Bahrain".to_string(),
            isdst: false
        }, Timezone {
            offset: 5.0,
            name: "Asia/Baku".to_string(),
            isdst: true
        }, Timezone {
            offset: 7.0,
            name: "Asia/Bangkok".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Asia/Beirut".to_string(),
            isdst: true
        }, Timezone {
            offset: 6.0,
            name: "Asia/Bishkek".to_string(),
            isdst: false
        }, Timezone {
            offset: 8.0,
            name: "Asia/Brunei".to_string(),
            isdst: false
        }, Timezone {
            offset: 5.5,
            name: "Asia/Calcutta".to_string(),
            isdst: false
        }, Timezone {
            offset: 9.0,
            name: "Asia/Chita".to_string(),
            isdst: false
        }, Timezone {
            offset: 8.0,
            name: "Asia/Choibalsan".to_string(),
            isdst: false
        }, Timezone {
            offset: 5.5,
            name: "Asia/Colombo".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Asia/Damascus".to_string(),
            isdst: true
        }, Timezone {
            offset: 6.0,
            name: "Asia/Dhaka".to_string(),
            isdst: false
        }, Timezone {
            offset: 9.0,
            name: "Asia/Dili".to_string(),
            isdst: false
        }, Timezone {
            offset: 4.0,
            name: "Asia/Dubai".to_string(),
            isdst: false
        }, Timezone {
            offset: 5.0,
            name: "Asia/Dushanbe".to_string(),
            isdst: false
        }, Timezone {
            offset: 8.0,
            name: "Asia/Hong_Kong".to_string(),
            isdst: false
        }, Timezone {
            offset: 7.0,
            name: "Asia/Hovd".to_string(),
            isdst: false
        }, Timezone {
            offset: 8.0,
            name: "Asia/Irkutsk".to_string(),
            isdst: false
        }, Timezone {
            offset: 7.0,
            name: "Asia/Jakarta".to_string(),
            isdst: false
        }, Timezone {
            offset: 9.0,
            name: "Asia/Jayapura".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Asia/Jerusalem".to_string(),
            isdst: true
        }, Timezone {
            offset: 4.5,
            name: "Asia/Kabul".to_string(),
            isdst: false
        }, Timezone {
            offset: 12.0,
            name: "Asia/Kamchatka".to_string(),
            isdst: false
        }, Timezone {
            offset: 13.0,
            name: "Asia/Kamchatka".to_string(),
            isdst: true
        }, Timezone {
            offset: 5.0,
            name: "Asia/Karachi".to_string(),
            isdst: false
        }, Timezone {
            offset: 5.75,
            name: "Asia/Kathmandu".to_string(),
            isdst: false
        }, Timezone {
            offset: 9.0,
            name: "Asia/Khandyga".to_string(),
            isdst: false
        }, Timezone {
            offset: 5.5,
            name: "Asia/Kolkata".to_string(),
            isdst: false
        }, Timezone {
            offset: 8.0,
            name: "Asia/Krasnoyarsk".to_string(),
            isdst: false
        }, Timezone {
            offset: 8.0,
            name: "Asia/Kuala_Lumpur".to_string(),
            isdst: false
        }, Timezone {
            offset: 8.0,
            name: "Asia/Kuching".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Asia/Kuwait".to_string(),
            isdst: false
        }, Timezone {
            offset: 8.0,
            name: "Asia/Macau".to_string(),
            isdst: false
        }, Timezone {
            offset: 12.0,
            name: "Asia/Magadan".to_string(),
            isdst: false
        }, Timezone {
            offset: 8.0,
            name: "Asia/Makassar".to_string(),
            isdst: false
        }, Timezone {
            offset: 8.0,
            name: "Asia/Manila".to_string(),
            isdst: false
        }, Timezone {
            offset: 4.0,
            name: "Asia/Muscat".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Asia/Nicosia".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Asia/Nicosia".to_string(),
            isdst: true
        }, Timezone {
            offset: 7.0,
            name: "Asia/Novokuznetsk".to_string(),
            isdst: false
        }, Timezone {
            offset: 7.0,
            name: "Asia/Novosibirsk".to_string(),
            isdst: false
        }, Timezone {
            offset: 7.0,
            name: "Asia/Omsk".to_string(),
            isdst: false
        }, Timezone {
            offset: 5.0,
            name: "Asia/Oral".to_string(),
            isdst: false
        }, Timezone {
            offset: 7.0,
            name: "Asia/Phnom_Penh".to_string(),
            isdst: false
        }, Timezone {
            offset: 7.0,
            name: "Asia/Pontianak".to_string(),
            isdst: false
        }, Timezone {
            offset: 9.0,
            name: "Asia/Pyongyang".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Asia/Qatar".to_string(),
            isdst: false
        }, Timezone {
            offset: 6.0,
            name: "Asia/Qyzylorda".to_string(),
            isdst: false
        }, Timezone {
            offset: 6.5,
            name: "Asia/Rangoon".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Asia/Riyadh".to_string(),
            isdst: false
        }, Timezone {
            offset: 7.0,
            name: "Asia/Saigon".to_string(),
            isdst: false
        }, Timezone {
            offset: 11.0,
            name: "Asia/Sakhalin".to_string(),
            isdst: false
        }, Timezone {
            offset: 5.0,
            name: "Asia/Samarkand".to_string(),
            isdst: false
        }, Timezone {
            offset: 9.0,
            name: "Asia/Seoul".to_string(),
            isdst: false
        }, Timezone {
            offset: 8.0,
            name: "Asia/Shanghai".to_string(),
            isdst: false
        }, Timezone {
            offset: 8.0,
            name: "Asia/Singapore".to_string(),
            isdst: false
        }, Timezone {
            offset: 12.0,
            name: "Asia/Srednekolymsk".to_string(),
            isdst: false
        }, Timezone {
            offset: 8.0,
            name: "Asia/Taipei".to_string(),
            isdst: false
        }, Timezone {
            offset: 5.0,
            name: "Asia/Tashkent".to_string(),
            isdst: false
        }, Timezone {
            offset: 4.0,
            name: "Asia/Tbilisi".to_string(),
            isdst: false
        }, Timezone {
            offset: 4.5,
            name: "Asia/Tehran".to_string(),
            isdst: true
        }, Timezone {
            offset: 6.0,
            name: "Asia/Thimphu".to_string(),
            isdst: false
        }, Timezone {
            offset: 9.0,
            name: "Asia/Tokyo".to_string(),
            isdst: false
        }, Timezone {
            offset: 8.0,
            name: "Asia/Ulaanbaatar".to_string(),
            isdst: false
        }, Timezone {
            offset: 6.0,
            name: "Asia/Urumqi".to_string(),
            isdst: false
        }, Timezone {
            offset: 11.0,
            name: "Asia/Ust-Nera".to_string(),
            isdst: false
        }, Timezone {
            offset: 7.0,
            name: "Asia/Vientiane".to_string(),
            isdst: false
        }, Timezone {
            offset: 11.0,
            name: "Asia/Vladivostok".to_string(),
            isdst: false
        }, Timezone {
            offset: 9.0,
            name: "Asia/Yakutsk".to_string(),
            isdst: false
        }, Timezone {
            offset: 5.0,
            name: "Asia/Yekaterinburg".to_string(),
            isdst: false
        }, Timezone {
            offset: 4.0,
            name: "Asia/Yerevan".to_string(),
            isdst: false
        }, Timezone {
            offset: 0.0,
            name: "Atlantic/Azores".to_string(),
            isdst: true
        }, Timezone {
            offset: -3.0,
            name: "Atlantic/Bermuda".to_string(),
            isdst: true
        }, Timezone {
            offset: 1.0,
            name: "Atlantic/Canary".to_string(),
            isdst: true
        }, Timezone {
            offset: -1.0,
            name: "Atlantic/Cape_Verde".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Atlantic/Faeroe".to_string(),
            isdst: true
        }, Timezone {
            offset: 1.0,
            name: "Atlantic/Madeira".to_string(),
            isdst: true
        }, Timezone {
            offset: 0.0,
            name: "Atlantic/Reykjavik".to_string(),
            isdst: false
        }, Timezone {
            offset: -2.0,
            name: "Atlantic/South_Georgia".to_string(),
            isdst: false
        }, Timezone {
            offset: 0.0,
            name: "Atlantic/St_Helena".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "Atlantic/Stanley".to_string(),
            isdst: false
        }, Timezone {
            offset: 9.5,
            name: "Australia/Adelaide".to_string(),
            isdst: false
        }, Timezone {
            offset: 10.0,
            name: "Australia/Brisbane".to_string(),
            isdst: false
        }, Timezone {
            offset: 9.5,
            name: "Australia/Broken_Hill".to_string(),
            isdst: false
        }, Timezone {
            offset: 10.0,
            name: "Australia/Currie".to_string(),
            isdst: false
        }, Timezone {
            offset: 9.5,
            name: "Australia/Darwin".to_string(),
            isdst: false
        }, Timezone {
            offset: 10.0,
            name: "Australia/Hobart".to_string(),
            isdst: false
        }, Timezone {
            offset: 10.0,
            name: "Australia/Lindeman".to_string(),
            isdst: false
        }, Timezone {
            offset: 10.0,
            name: "Australia/Melbourne".to_string(),
            isdst: false
        }, Timezone {
            offset: 8.0,
            name: "Australia/Perth".to_string(),
            isdst: false
        }, Timezone {
            offset: 10.0,
            name: "Australia/Sydney".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "CST6CDT".to_string(),
            isdst: true
        }, Timezone {
            offset: 0.0,
            name: "Etc/GMT".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Etc/GMT-1".to_string(),
            isdst: false
        }, Timezone {
            offset: 10.0,
            name: "Etc/GMT-10".to_string(),
            isdst: false
        }, Timezone {
            offset: 11.0,
            name: "Etc/GMT-11".to_string(),
            isdst: false
        }, Timezone {
            offset: 12.0,
            name: "Etc/GMT-12".to_string(),
            isdst: false
        }, Timezone {
            offset: 13.0,
            name: "Etc/GMT-13".to_string(),
            isdst: false
        }, Timezone {
            offset: 2.0,
            name: "Etc/GMT-2".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Etc/GMT-3".to_string(),
            isdst: false
        }, Timezone {
            offset: 4.0,
            name: "Etc/GMT-4".to_string(),
            isdst: false
        }, Timezone {
            offset: 5.0,
            name: "Etc/GMT-5".to_string(),
            isdst: false
        }, Timezone {
            offset: 6.0,
            name: "Etc/GMT-6".to_string(),
            isdst: false
        }, Timezone {
            offset: 7.0,
            name: "Etc/GMT-7".to_string(),
            isdst: false
        }, Timezone {
            offset: 8.0,
            name: "Etc/GMT-8".to_string(),
            isdst: false
        }, Timezone {
            offset: 9.0,
            name: "Etc/GMT-9".to_string(),
            isdst: false
        }, Timezone {
            offset: -1.0,
            name: "Etc/GMT+1".to_string(),
            isdst: false
        }, Timezone {
            offset: -10.0,
            name: "Etc/GMT+10".to_string(),
            isdst: false
        }, Timezone {
            offset: -11.0,
            name: "Etc/GMT+11".to_string(),
            isdst: false
        }, Timezone {
            offset: -12.0,
            name: "Etc/GMT+12".to_string(),
            isdst: false
        }, Timezone {
            offset: -2.0,
            name: "Etc/GMT+2".to_string(),
            isdst: false
        }, Timezone {
            offset: -3.0,
            name: "Etc/GMT+3".to_string(),
            isdst: false
        }, Timezone {
            offset: -4.0,
            name: "Etc/GMT+4".to_string(),
            isdst: false
        }, Timezone {
            offset: -5.0,
            name: "Etc/GMT+5".to_string(),
            isdst: false
        }, Timezone {
            offset: -6.0,
            name: "Etc/GMT+6".to_string(),
            isdst: false
        }, Timezone {
            offset: -7.0,
            name: "Etc/GMT+7".to_string(),
            isdst: false
        }, Timezone {
            offset: 2.0,
            name: "Europe/Amsterdam".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Andorra".to_string(),
            isdst: true
        }, Timezone {
            offset: 4.0,
            name: "Europe/Astrakhan".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Europe/Athens".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Athens".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Belgrade".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Berlin".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Bratislava".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Brussels".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Bucharest".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Bucharest".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Budapest".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Busingen".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Chisinau".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Chisinau".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Copenhagen".to_string(),
            isdst: true
        }, Timezone {
            offset: 1.0,
            name: "Europe/Dublin".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Gibraltar".to_string(),
            isdst: true
        }, Timezone {
            offset: 0.0,
            name: "Europe/Guernsey".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Europe/Guernsey".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Helsinki".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Helsinki".to_string(),
            isdst: true
        }, Timezone {
            offset: 0.0,
            name: "Europe/Isle_of_Man".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Europe/Isle_of_Man".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Istanbul".to_string(),
            isdst: false
        }, Timezone {
            offset: 0.0,
            name: "Europe/Jersey".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Europe/Jersey".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Kaliningrad".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Europe/Kiev".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Kiev".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Kirov".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Europe/Lisbon".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Ljubljana".to_string(),
            isdst: true
        }, Timezone {
            offset: 0.0,
            name: "Europe/London".to_string(),
            isdst: false
        }, Timezone {
            offset: 1.0,
            name: "Europe/London".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Luxembourg".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Madrid".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Malta".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Mariehamn".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Mariehamn".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Minsk".to_string(),
            isdst: false
        }, Timezone {
            offset: 2.0,
            name: "Europe/Monaco".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Moscow".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Europe/Nicosia".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Oslo".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Paris".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Podgorica".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Prague".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Riga".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Riga".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Rome".to_string(),
            isdst: true
        }, Timezone {
            offset: 4.0,
            name: "Europe/Samara".to_string(),
            isdst: false
        }, Timezone {
            offset: 2.0,
            name: "Europe/San_Marino".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Sarajevo".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Simferopol".to_string(),
            isdst: false
        }, Timezone {
            offset: 2.0,
            name: "Europe/Skopje".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Sofia".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Sofia".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Stockholm".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Tallinn".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Tallinn".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Tirane".to_string(),
            isdst: true
        }, Timezone {
            offset: 4.0,
            name: "Europe/Ulyanovsk".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Europe/Uzhgorod".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Uzhgorod".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Vaduz".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Vatican".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Vienna".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Vilnius".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Vilnius".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Volgograd".to_string(),
            isdst: false
        }, Timezone {
            offset: 2.0,
            name: "Europe/Warsaw".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Zagreb".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Zaporozhye".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Europe/Zaporozhye".to_string(),
            isdst: true
        }, Timezone {
            offset: 2.0,
            name: "Europe/Zurich".to_string(),
            isdst: true
        }, Timezone {
            offset: 3.0,
            name: "Indian/Antananarivo".to_string(),
            isdst: false
        }, Timezone {
            offset: 6.0,
            name: "Indian/Chagos".to_string(),
            isdst: false
        }, Timezone {
            offset: 7.0,
            name: "Indian/Christmas".to_string(),
            isdst: false
        }, Timezone {
            offset: 6.5,
            name: "Indian/Cocos".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Indian/Comoro".to_string(),
            isdst: false
        }, Timezone {
            offset: 5.0,
            name: "Indian/Kerguelen".to_string(),
            isdst: false
        }, Timezone {
            offset: 4.0,
            name: "Indian/Mahe".to_string(),
            isdst: false
        }, Timezone {
            offset: 5.0,
            name: "Indian/Maldives".to_string(),
            isdst: false
        }, Timezone {
            offset: 4.0,
            name: "Indian/Mauritius".to_string(),
            isdst: false
        }, Timezone {
            offset: 3.0,
            name: "Indian/Mayotte".to_string(),
            isdst: false
        }, Timezone {
            offset: 4.0,
            name: "Indian/Reunion".to_string(),
            isdst: false
        }, Timezone {
            offset: -6.0,
            name: "MST7MDT".to_string(),
            isdst: true
        }, Timezone {
            offset: 13.0,
            name: "Pacific/Apia".to_string(),
            isdst: false
        }, Timezone {
            offset: 12.0,
            name: "Pacific/Auckland".to_string(),
            isdst: false
        }, Timezone {
            offset: 11.0,
            name: "Pacific/Efate".to_string(),
            isdst: false
        }, Timezone {
            offset: 13.0,
            name: "Pacific/Enderbury".to_string(),
            isdst: false
        }, Timezone {
            offset: 13.0,
            name: "Pacific/Fakaofo".to_string(),
            isdst: false
        }, Timezone {
            offset: 12.0,
            name: "Pacific/Fiji".to_string(),
            isdst: false
        }, Timezone {
            offset: 12.0,
            name: "Pacific/Funafuti".to_string(),
            isdst: false
        }, Timezone {
            offset: -6.0,
            name: "Pacific/Galapagos".to_string(),
            isdst: false
        }, Timezone {
            offset: 11.0,
            name: "Pacific/Guadalcanal".to_string(),
            isdst: false
        }, Timezone {
            offset: 10.0,
            name: "Pacific/Guam".to_string(),
            isdst: false
        }, Timezone {
            offset: -10.0,
            name: "Pacific/Honolulu".to_string(),
            isdst: false
        }, Timezone {
            offset: -10.0,
            name: "Pacific/Johnston".to_string(),
            isdst: false
        }, Timezone {
            offset: 11.0,
            name: "Pacific/Kosrae".to_string(),
            isdst: false
        }, Timezone {
            offset: 12.0,
            name: "Pacific/Kwajalein".to_string(),
            isdst: false
        }, Timezone {
            offset: 12.0,
            name: "Pacific/Majuro".to_string(),
            isdst: false
        }, Timezone {
            offset: -11.0,
            name: "Pacific/Midway".to_string(),
            isdst: false
        }, Timezone {
            offset: 12.0,
            name: "Pacific/Nauru".to_string(),
            isdst: false
        }, Timezone {
            offset: -11.0,
            name: "Pacific/Niue".to_string(),
            isdst: false
        }, Timezone {
            offset: 11.0,
            name: "Pacific/Noumea".to_string(),
            isdst: false
        }, Timezone {
            offset: -11.0,
            name: "Pacific/Pago_Pago".to_string(),
            isdst: false
        }, Timezone {
            offset: 9.0,
            name: "Pacific/Palau".to_string(),
            isdst: false
        }, Timezone {
            offset: 11.0,
            name: "Pacific/Ponape".to_string(),
            isdst: false
        }, Timezone {
            offset: 10.0,
            name: "Pacific/Port_Moresby".to_string(),
            isdst: false
        }, Timezone {
            offset: -10.0,
            name: "Pacific/Rarotonga".to_string(),
            isdst: false
        }, Timezone {
            offset: 10.0,
            name: "Pacific/Saipan".to_string(),
            isdst: false
        }, Timezone {
            offset: -10.0,
            name: "Pacific/Tahiti".to_string(),
            isdst: false
        }, Timezone {
            offset: 12.0,
            name: "Pacific/Tarawa".to_string(),
            isdst: false
        }, Timezone {
            offset: 13.0,
            name: "Pacific/Tongatapu".to_string(),
            isdst: false
        }, Timezone {
            offset: 10.0,
            name: "Pacific/Truk".to_string(),
            isdst: false
        }, Timezone {
            offset: 12.0,
            name: "Pacific/Wake".to_string(),
            isdst: false
        }, Timezone {
            offset: 12.0,
            name: "Pacific/Wallis".to_string(),
            isdst: false
        }, Timezone {
            offset: -8.0,
            name: "PST8PDT".to_string(),
            isdst: false
        }, Timezone {
            offset: 0.0,
            name: "UTC".to_string(),
            isdst: false
        }];
        m
    };
}

#[cfg(test)]
#[test]
fn timezone_list_test() {
    use crate::{calculation::Calculation, settings::Settings};

    let mut calculation = Calculation::default();
    let mut settings = Settings::default();
    for timezone in TIMEZONE_LIST.iter() {
        settings.timezone = timezone.clone();
        calculation.configure(&settings);
        assert_eq!(calculation.smartcalc.get_time_offset().offset/60, timezone.offset as i32);
    }
}
