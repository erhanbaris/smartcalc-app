use lazy_static::*;

#[derive(Default, PartialEq)]
#[derive(Debug, Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Timezone {
    pub name: String,
    pub data: String
}

impl ToString for Timezone {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
}

lazy_static! {
    pub static ref TIMEZONE_LIST: Vec<Timezone> = {
        let m = vec![Timezone {
            data: "GMT0".to_string(),
            name: "Africa/Abidjan".to_string()
        }, Timezone {
            data: "GMT0".to_string(),
            name: "Africa/Accra".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Africa/Addis_Ababa".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Africa/Algiers".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Africa/Asmera".to_string()
        }, Timezone {
            data: "GMT0".to_string(),
            name: "Africa/Bamako".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Africa/Bangui".to_string()
        }, Timezone {
            data: "GMT0".to_string(),
            name: "Africa/Banjul".to_string()
        }, Timezone {
            data: "GMT0".to_string(),
            name: "Africa/Bissau".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Africa/Blantyre".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Africa/Brazzaville".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Africa/Bujumbura".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Africa/Cairo".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Africa/Casablanca".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Africa/Ceuta".to_string()
        }, Timezone {
            data: "GMT0".to_string(),
            name: "Africa/Conakry".to_string()
        }, Timezone {
            data: "GMT0".to_string(),
            name: "Africa/Dakar".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Africa/Dar_es_Salaam".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Africa/Djibouti".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Africa/Douala".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Africa/El_Aaiun".to_string()
        }, Timezone {
            data: "GMT0".to_string(),
            name: "Africa/Freetown".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Africa/Gaborone".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Africa/Harare".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Africa/Johannesburg".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Africa/Juba".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Africa/Kampala".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Africa/Khartoum".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Africa/Kigali".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Africa/Kinshasa".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Africa/Lagos".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Africa/Libreville".to_string()
        }, Timezone {
            data: "GMT0".to_string(),
            name: "Africa/Lome".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Africa/Luanda".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Africa/Lubumbashi".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Africa/Lusaka".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Africa/Malabo".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Africa/Maputo".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Africa/Maseru".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Africa/Mbabane".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Africa/Mogadishu".to_string()
        }, Timezone {
            data: "GMT0".to_string(),
            name: "Africa/Monrovia".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Africa/Nairobi".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Africa/Ndjamena".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Africa/Niamey".to_string()
        }, Timezone {
            data: "GMT0".to_string(),
            name: "Africa/Nouakchott".to_string()
        }, Timezone {
            data: "GMT0".to_string(),
            name: "Africa/Ouagadougou".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Africa/Porto-Novo".to_string()
        }, Timezone {
            data: "GMT0".to_string(),
            name: "Africa/Sao_Tome".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Africa/Tripoli".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Africa/Tunis".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Africa/Windhoek".to_string()
        }, Timezone {
            data: "GMT-8".to_string(),
            name: "America/Anchorage".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Anguilla".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Antigua".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Araguaina".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Argentina/La_Rioja".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Argentina/Rio_Gallegos".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Argentina/Salta".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Argentina/San_Juan".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Argentina/San_Luis".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Argentina/Tucuman".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Argentina/Ushuaia".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Aruba".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Asuncion".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Bahia".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Bahia_Banderas".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Barbados".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Belem".to_string()
        }, Timezone {
            data: "GMT-6".to_string(),
            name: "America/Belize".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Blanc-Sablon".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Boa_Vista".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Bogota".to_string()
        }, Timezone {
            data: "GMT-6".to_string(),
            name: "America/Boise".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Buenos_Aires".to_string()
        }, Timezone {
            data: "GMT-6".to_string(),
            name: "America/Cambridge_Bay".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Campo_Grande".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Cancun".to_string()
        }, Timezone {
            data: "GMT-4:30".to_string(),
            name: "America/Caracas".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Catamarca".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Cayenne".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Cayman".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Chicago".to_string()
        }, Timezone {
            data: "GMT-6".to_string(),
            name: "America/Chihuahua".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Coral_Harbour".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Cordoba".to_string()
        }, Timezone {
            data: "GMT-6".to_string(),
            name: "America/Costa_Rica".to_string()
        }, Timezone {
            data: "GMT-7".to_string(),
            name: "America/Creston".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Cuiaba".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Curacao".to_string()
        }, Timezone {
            data: "GMT0".to_string(),
            name: "America/Danmarkshavn".to_string()
        }, Timezone {
            data: "GMT-7".to_string(),
            name: "America/Dawson".to_string()
        }, Timezone {
            data: "GMT-7".to_string(),
            name: "America/Dawson_Creek".to_string()
        }, Timezone {
            data: "GMT-6".to_string(),
            name: "America/Denver".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Detroit".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Detroit".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Dominica".to_string()
        }, Timezone {
            data: "GMT-6".to_string(),
            name: "America/Edmonton".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Eirunepe".to_string()
        }, Timezone {
            data: "GMT-6".to_string(),
            name: "America/El_Salvador".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Fortaleza".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Glace_Bay".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Godthab".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Goose_Bay".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Grand_Turk".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Grenada".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Guadeloupe".to_string()
        }, Timezone {
            data: "GMT-6".to_string(),
            name: "America/Guatemala".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Guayaquil".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Guyana".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Halifax".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Havana".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Havana".to_string()
        }, Timezone {
            data: "GMT-7".to_string(),
            name: "America/Hermosillo".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Indiana/Knox".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Indiana/Marengo".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Indiana/Petersburg".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Indiana/Petersburg".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Indiana/Tell_City".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Indiana/Vevay".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Indiana/Vincennes".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Indiana/Vincennes".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Indiana/Winamac".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Indiana/Winamac".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Indianapolis".to_string()
        }, Timezone {
            data: "GMT-6".to_string(),
            name: "America/Inuvik".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Iqaluit".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Iqaluit".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Jamaica".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Jujuy".to_string()
        }, Timezone {
            data: "GMT-8".to_string(),
            name: "America/Juneau".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Kentucky/Monticello".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Kentucky/Monticello".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Kralendijk".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/La_Paz".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Lima".to_string()
        }, Timezone {
            data: "GMT-7".to_string(),
            name: "America/Los_Angeles".to_string()
        }, Timezone {
            data: "GMT-8".to_string(),
            name: "America/Los_Angeles".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Louisville".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Louisville".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Lower_Princes".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Maceio".to_string()
        }, Timezone {
            data: "GMT-6".to_string(),
            name: "America/Managua".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Manaus".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Marigot".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Martinique".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Matamoros".to_string()
        }, Timezone {
            data: "GMT-6".to_string(),
            name: "America/Mazatlan".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Mendoza".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Menominee".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Merida".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Mexico_City".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Moncton".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Monterrey".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Montevideo".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Montreal".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Montreal".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Montserrat".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Nassau".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Nassau".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/New_York".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/New_York".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Nipigon".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Nipigon".to_string()
        }, Timezone {
            data: "GMT-8".to_string(),
            name: "America/Nome".to_string()
        }, Timezone {
            data: "GMT-2".to_string(),
            name: "America/Noronha".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/North_Dakota/Beulah".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/North_Dakota/Center".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/North_Dakota/New_Salem".to_string()
        }, Timezone {
            data: "GMT-6".to_string(),
            name: "America/Ojinaga".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Panama".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Pangnirtung".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Pangnirtung".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Paramaribo".to_string()
        }, Timezone {
            data: "GMT-7".to_string(),
            name: "America/Phoenix".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Port_of_Spain".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Port-au-Prince".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Port-au-Prince".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Porto_Velho".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Puerto_Rico".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Rainy_River".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Rankin_Inlet".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Recife".to_string()
        }, Timezone {
            data: "GMT-6".to_string(),
            name: "America/Regina".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Resolute".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Rio_Branco".to_string()
        }, Timezone {
            data: "GMT-7".to_string(),
            name: "America/Santa_Isabel".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Santarem".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Santiago".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Santo_Domingo".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Sao_Paulo".to_string()
        }, Timezone {
            data: "GMT0".to_string(),
            name: "America/Scoresbysund".to_string()
        }, Timezone {
            data: "GMT-8".to_string(),
            name: "America/Sitka".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/St_Barthelemy".to_string()
        }, Timezone {
            data: "GMT-2:30".to_string(),
            name: "America/St_Johns".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/St_Kitts".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/St_Lucia".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/St_Thomas".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/St_Vincent".to_string()
        }, Timezone {
            data: "GMT-6".to_string(),
            name: "America/Swift_Current".to_string()
        }, Timezone {
            data: "GMT-6".to_string(),
            name: "America/Tegucigalpa".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "America/Thule".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Thunder_Bay".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Thunder_Bay".to_string()
        }, Timezone {
            data: "GMT-7".to_string(),
            name: "America/Tijuana".to_string()
        }, Timezone {
            data: "GMT-8".to_string(),
            name: "America/Tijuana".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Toronto".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Toronto".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "America/Tortola".to_string()
        }, Timezone {
            data: "GMT-7".to_string(),
            name: "America/Vancouver".to_string()
        }, Timezone {
            data: "GMT-8".to_string(),
            name: "America/Vancouver".to_string()
        }, Timezone {
            data: "GMT-7".to_string(),
            name: "America/Whitehorse".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "America/Winnipeg".to_string()
        }, Timezone {
            data: "GMT-8".to_string(),
            name: "America/Yakutat".to_string()
        }, Timezone {
            data: "GMT-6".to_string(),
            name: "America/Yellowknife".to_string()
        }, Timezone {
            data: "GMT8".to_string(),
            name: "Antarctica/Casey".to_string()
        }, Timezone {
            data: "GMT7".to_string(),
            name: "Antarctica/Davis".to_string()
        }, Timezone {
            data: "GMT10".to_string(),
            name: "Antarctica/DumontDUrville".to_string()
        }, Timezone {
            data: "GMT11".to_string(),
            name: "Antarctica/Macquarie".to_string()
        }, Timezone {
            data: "GMT5".to_string(),
            name: "Antarctica/Mawson".to_string()
        }, Timezone {
            data: "GMT12".to_string(),
            name: "Antarctica/McMurdo".to_string()
        }, Timezone {
            data: "GMT-4".to_string(),
            name: "Antarctica/Palmer".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "Antarctica/Rothera".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Antarctica/Syowa".to_string()
        }, Timezone {
            data: "GMT6".to_string(),
            name: "Antarctica/Vostok".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Arctic/Longyearbyen".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Asia/Aden".to_string()
        }, Timezone {
            data: "GMT6".to_string(),
            name: "Asia/Almaty".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Asia/Amman".to_string()
        }, Timezone {
            data: "GMT12".to_string(),
            name: "Asia/Anadyr".to_string()
        }, Timezone {
            data: "GMT5".to_string(),
            name: "Asia/Aqtau".to_string()
        }, Timezone {
            data: "GMT5".to_string(),
            name: "Asia/Aqtobe".to_string()
        }, Timezone {
            data: "GMT5".to_string(),
            name: "Asia/Ashgabat".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Asia/Baghdad".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Asia/Bahrain".to_string()
        }, Timezone {
            data: "GMT5".to_string(),
            name: "Asia/Baku".to_string()
        }, Timezone {
            data: "GMT7".to_string(),
            name: "Asia/Bangkok".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Asia/Beirut".to_string()
        }, Timezone {
            data: "GMT6".to_string(),
            name: "Asia/Bishkek".to_string()
        }, Timezone {
            data: "GMT8".to_string(),
            name: "Asia/Brunei".to_string()
        }, Timezone {
            data: "GMT5:30".to_string(),
            name: "Asia/Calcutta".to_string()
        }, Timezone {
            data: "GMT9".to_string(),
            name: "Asia/Chita".to_string()
        }, Timezone {
            data: "GMT8".to_string(),
            name: "Asia/Choibalsan".to_string()
        }, Timezone {
            data: "GMT5:30".to_string(),
            name: "Asia/Colombo".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Asia/Damascus".to_string()
        }, Timezone {
            data: "GMT6".to_string(),
            name: "Asia/Dhaka".to_string()
        }, Timezone {
            data: "GMT9".to_string(),
            name: "Asia/Dili".to_string()
        }, Timezone {
            data: "GMT4".to_string(),
            name: "Asia/Dubai".to_string()
        }, Timezone {
            data: "GMT5".to_string(),
            name: "Asia/Dushanbe".to_string()
        }, Timezone {
            data: "GMT8".to_string(),
            name: "Asia/Hong_Kong".to_string()
        }, Timezone {
            data: "GMT7".to_string(),
            name: "Asia/Hovd".to_string()
        }, Timezone {
            data: "GMT8".to_string(),
            name: "Asia/Irkutsk".to_string()
        }, Timezone {
            data: "GMT7".to_string(),
            name: "Asia/Jakarta".to_string()
        }, Timezone {
            data: "GMT9".to_string(),
            name: "Asia/Jayapura".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Asia/Jerusalem".to_string()
        }, Timezone {
            data: "GMT4:30".to_string(),
            name: "Asia/Kabul".to_string()
        }, Timezone {
            data: "GMT12".to_string(),
            name: "Asia/Kamchatka".to_string()
        }, Timezone {
            data: "GMT13".to_string(),
            name: "Asia/Kamchatka".to_string()
        }, Timezone {
            data: "GMT5".to_string(),
            name: "Asia/Karachi".to_string()
        }, Timezone {
            data: "GMT5:45".to_string(),
            name: "Asia/Kathmandu".to_string()
        }, Timezone {
            data: "GMT9".to_string(),
            name: "Asia/Khandyga".to_string()
        }, Timezone {
            data: "GMT5:30".to_string(),
            name: "Asia/Kolkata".to_string()
        }, Timezone {
            data: "GMT8".to_string(),
            name: "Asia/Krasnoyarsk".to_string()
        }, Timezone {
            data: "GMT8".to_string(),
            name: "Asia/Kuala_Lumpur".to_string()
        }, Timezone {
            data: "GMT8".to_string(),
            name: "Asia/Kuching".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Asia/Kuwait".to_string()
        }, Timezone {
            data: "GMT8".to_string(),
            name: "Asia/Macau".to_string()
        }, Timezone {
            data: "GMT12".to_string(),
            name: "Asia/Magadan".to_string()
        }, Timezone {
            data: "GMT8".to_string(),
            name: "Asia/Makassar".to_string()
        }, Timezone {
            data: "GMT8".to_string(),
            name: "Asia/Manila".to_string()
        }, Timezone {
            data: "GMT4".to_string(),
            name: "Asia/Muscat".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Asia/Nicosia".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Asia/Nicosia".to_string()
        }, Timezone {
            data: "GMT7".to_string(),
            name: "Asia/Novokuznetsk".to_string()
        }, Timezone {
            data: "GMT7".to_string(),
            name: "Asia/Novosibirsk".to_string()
        }, Timezone {
            data: "GMT7".to_string(),
            name: "Asia/Omsk".to_string()
        }, Timezone {
            data: "GMT5".to_string(),
            name: "Asia/Oral".to_string()
        }, Timezone {
            data: "GMT7".to_string(),
            name: "Asia/Phnom_Penh".to_string()
        }, Timezone {
            data: "GMT7".to_string(),
            name: "Asia/Pontianak".to_string()
        }, Timezone {
            data: "GMT9".to_string(),
            name: "Asia/Pyongyang".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Asia/Qatar".to_string()
        }, Timezone {
            data: "GMT6".to_string(),
            name: "Asia/Qyzylorda".to_string()
        }, Timezone {
            data: "GMT6:30".to_string(),
            name: "Asia/Rangoon".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Asia/Riyadh".to_string()
        }, Timezone {
            data: "GMT7".to_string(),
            name: "Asia/Saigon".to_string()
        }, Timezone {
            data: "GMT11".to_string(),
            name: "Asia/Sakhalin".to_string()
        }, Timezone {
            data: "GMT5".to_string(),
            name: "Asia/Samarkand".to_string()
        }, Timezone {
            data: "GMT9".to_string(),
            name: "Asia/Seoul".to_string()
        }, Timezone {
            data: "GMT8".to_string(),
            name: "Asia/Shanghai".to_string()
        }, Timezone {
            data: "GMT8".to_string(),
            name: "Asia/Singapore".to_string()
        }, Timezone {
            data: "GMT12".to_string(),
            name: "Asia/Srednekolymsk".to_string()
        }, Timezone {
            data: "GMT8".to_string(),
            name: "Asia/Taipei".to_string()
        }, Timezone {
            data: "GMT5".to_string(),
            name: "Asia/Tashkent".to_string()
        }, Timezone {
            data: "GMT4".to_string(),
            name: "Asia/Tbilisi".to_string()
        }, Timezone {
            data: "GMT4:30".to_string(),
            name: "Asia/Tehran".to_string()
        }, Timezone {
            data: "GMT6".to_string(),
            name: "Asia/Thimphu".to_string()
        }, Timezone {
            data: "GMT9".to_string(),
            name: "Asia/Tokyo".to_string()
        }, Timezone {
            data: "GMT8".to_string(),
            name: "Asia/Ulaanbaatar".to_string()
        }, Timezone {
            data: "GMT6".to_string(),
            name: "Asia/Urumqi".to_string()
        }, Timezone {
            data: "GMT11".to_string(),
            name: "Asia/Ust-Nera".to_string()
        }, Timezone {
            data: "GMT7".to_string(),
            name: "Asia/Vientiane".to_string()
        }, Timezone {
            data: "GMT11".to_string(),
            name: "Asia/Vladivostok".to_string()
        }, Timezone {
            data: "GMT9".to_string(),
            name: "Asia/Yakutsk".to_string()
        }, Timezone {
            data: "GMT5".to_string(),
            name: "Asia/Yekaterinburg".to_string()
        }, Timezone {
            data: "GMT4".to_string(),
            name: "Asia/Yerevan".to_string()
        }, Timezone {
            data: "GMT0".to_string(),
            name: "Atlantic/Azores".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "Atlantic/Bermuda".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Atlantic/Canary".to_string()
        }, Timezone {
            data: "GMT-1".to_string(),
            name: "Atlantic/Cape_Verde".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Atlantic/Faeroe".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Atlantic/Madeira".to_string()
        }, Timezone {
            data: "GMT0".to_string(),
            name: "Atlantic/Reykjavik".to_string()
        }, Timezone {
            data: "GMT-2".to_string(),
            name: "Atlantic/South_Georgia".to_string()
        }, Timezone {
            data: "GMT0".to_string(),
            name: "Atlantic/St_Helena".to_string()
        }, Timezone {
            data: "GMT-3".to_string(),
            name: "Atlantic/Stanley".to_string()
        }, Timezone {
            data: "GMT9:30".to_string(),
            name: "Australia/Adelaide".to_string()
        }, Timezone {
            data: "GMT10".to_string(),
            name: "Australia/Brisbane".to_string()
        }, Timezone {
            data: "GMT9:30".to_string(),
            name: "Australia/Broken_Hill".to_string()
        }, Timezone {
            data: "GMT10".to_string(),
            name: "Australia/Currie".to_string()
        }, Timezone {
            data: "GMT9:30".to_string(),
            name: "Australia/Darwin".to_string()
        }, Timezone {
            data: "GMT10".to_string(),
            name: "Australia/Hobart".to_string()
        }, Timezone {
            data: "GMT10".to_string(),
            name: "Australia/Lindeman".to_string()
        }, Timezone {
            data: "GMT10".to_string(),
            name: "Australia/Melbourne".to_string()
        }, Timezone {
            data: "GMT8".to_string(),
            name: "Australia/Perth".to_string()
        }, Timezone {
            data: "GMT10".to_string(),
            name: "Australia/Sydney".to_string()
        }, Timezone {
            data: "GMT-5".to_string(),
            name: "CST6CDT".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Amsterdam".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Andorra".to_string()
        }, Timezone {
            data: "GMT4".to_string(),
            name: "Europe/Astrakhan".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Athens".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Athens".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Belgrade".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Berlin".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Bratislava".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Brussels".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Bucharest".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Bucharest".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Budapest".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Busingen".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Chisinau".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Chisinau".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Copenhagen".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Europe/Dublin".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Gibraltar".to_string()
        }, Timezone {
            data: "GMT0".to_string(),
            name: "Europe/Guernsey".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Europe/Guernsey".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Helsinki".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Helsinki".to_string()
        }, Timezone {
            data: "GMT0".to_string(),
            name: "Europe/Isle_of_Man".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Europe/Isle_of_Man".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Istanbul".to_string()
        }, Timezone {
            data: "GMT0".to_string(),
            name: "Europe/Jersey".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Europe/Jersey".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Kaliningrad".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Kiev".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Kiev".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Kirov".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Europe/Lisbon".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Ljubljana".to_string()
        }, Timezone {
            data: "GMT0".to_string(),
            name: "Europe/London".to_string()
        }, Timezone {
            data: "GMT1".to_string(),
            name: "Europe/London".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Luxembourg".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Madrid".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Malta".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Mariehamn".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Mariehamn".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Minsk".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Monaco".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Moscow".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Nicosia".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Oslo".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Paris".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Podgorica".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Prague".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Riga".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Riga".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Rome".to_string()
        }, Timezone {
            data: "GMT4".to_string(),
            name: "Europe/Samara".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/San_Marino".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Sarajevo".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Simferopol".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Skopje".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Sofia".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Sofia".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Stockholm".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Tallinn".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Tallinn".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Tirane".to_string()
        }, Timezone {
            data: "GMT4".to_string(),
            name: "Europe/Ulyanovsk".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Uzhgorod".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Uzhgorod".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Vaduz".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Vatican".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Vienna".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Vilnius".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Vilnius".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Volgograd".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Warsaw".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Zagreb".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Zaporozhye".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Europe/Zaporozhye".to_string()
        }, Timezone {
            data: "GMT2".to_string(),
            name: "Europe/Zurich".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Indian/Antananarivo".to_string()
        }, Timezone {
            data: "GMT6".to_string(),
            name: "Indian/Chagos".to_string()
        }, Timezone {
            data: "GMT7".to_string(),
            name: "Indian/Christmas".to_string()
        }, Timezone {
            data: "GMT6:30".to_string(),
            name: "Indian/Cocos".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Indian/Comoro".to_string()
        }, Timezone {
            data: "GMT5".to_string(),
            name: "Indian/Kerguelen".to_string()
        }, Timezone {
            data: "GMT4".to_string(),
            name: "Indian/Mahe".to_string()
        }, Timezone {
            data: "GMT5".to_string(),
            name: "Indian/Maldives".to_string()
        }, Timezone {
            data: "GMT4".to_string(),
            name: "Indian/Mauritius".to_string()
        }, Timezone {
            data: "GMT3".to_string(),
            name: "Indian/Mayotte".to_string()
        }, Timezone {
            data: "GMT4".to_string(),
            name: "Indian/Reunion".to_string()
        }, Timezone {
            data: "GMT-6".to_string(),
            name: "MST7MDT".to_string()
        }, Timezone {
            data: "GMT13".to_string(),
            name: "Pacific/Apia".to_string()
        }, Timezone {
            data: "GMT12".to_string(),
            name: "Pacific/Auckland".to_string()
        }, Timezone {
            data: "GMT11".to_string(),
            name: "Pacific/Efate".to_string()
        }, Timezone {
            data: "GMT13".to_string(),
            name: "Pacific/Enderbury".to_string()
        }, Timezone {
            data: "GMT13".to_string(),
            name: "Pacific/Fakaofo".to_string()
        }, Timezone {
            data: "GMT12".to_string(),
            name: "Pacific/Fiji".to_string()
        }, Timezone {
            data: "GMT12".to_string(),
            name: "Pacific/Funafuti".to_string()
        }, Timezone {
            data: "GMT-6".to_string(),
            name: "Pacific/Galapagos".to_string()
        }, Timezone {
            data: "GMT11".to_string(),
            name: "Pacific/Guadalcanal".to_string()
        }, Timezone {
            data: "GMT10".to_string(),
            name: "Pacific/Guam".to_string()
        }, Timezone {
            data: "GMT-10".to_string(),
            name: "Pacific/Honolulu".to_string()
        }, Timezone {
            data: "GMT-10".to_string(),
            name: "Pacific/Johnston".to_string()
        }, Timezone {
            data: "GMT11".to_string(),
            name: "Pacific/Kosrae".to_string()
        }, Timezone {
            data: "GMT12".to_string(),
            name: "Pacific/Kwajalein".to_string()
        }, Timezone {
            data: "GMT12".to_string(),
            name: "Pacific/Majuro".to_string()
        }, Timezone {
            data: "GMT-11".to_string(),
            name: "Pacific/Midway".to_string()
        }, Timezone {
            data: "GMT12".to_string(),
            name: "Pacific/Nauru".to_string()
        }, Timezone {
            data: "GMT-11".to_string(),
            name: "Pacific/Niue".to_string()
        }, Timezone {
            data: "GMT11".to_string(),
            name: "Pacific/Noumea".to_string()
        }, Timezone {
            data: "GMT-11".to_string(),
            name: "Pacific/Pago_Pago".to_string()
        }, Timezone {
            data: "GMT9".to_string(),
            name: "Pacific/Palau".to_string()
        }, Timezone {
            data: "GMT11".to_string(),
            name: "Pacific/Ponape".to_string()
        }, Timezone {
            data: "GMT10".to_string(),
            name: "Pacific/Port_Moresby".to_string()
        }, Timezone {
            data: "GMT-10".to_string(),
            name: "Pacific/Rarotonga".to_string()
        }, Timezone {
            data: "GMT10".to_string(),
            name: "Pacific/Saipan".to_string()
        }, Timezone {
            data: "GMT-10".to_string(),
            name: "Pacific/Tahiti".to_string()
        }, Timezone {
            data: "GMT12".to_string(),
            name: "Pacific/Tarawa".to_string()
        }, Timezone {
            data: "GMT13".to_string(),
            name: "Pacific/Tongatapu".to_string()
        }, Timezone {
            data: "GMT10".to_string(),
            name: "Pacific/Truk".to_string()
        }, Timezone {
            data: "GMT12".to_string(),
            name: "Pacific/Wake".to_string()
        }, Timezone {
            data: "GMT12".to_string(),
            name: "Pacific/Wallis".to_string()
        }, Timezone {
            data: "GMT-8".to_string(),
            name: "PST8PDT".to_string()
        }, Timezone {
            data: "UTC".to_string(),
            name: "UTC".to_string()
        }];
        m
    };
}
