# google_maps

An unofficial Google Maps Platform API for the Rust programming language.

# Welcome

As of version 0.1.0 this crate is expected to work well, work reliably, and
have the most important features implemented. There are some creature
comforts and specialized APIs not implemented yet.

While an early release, for most people this crate should work fine as is.

# Example Request:

```
use google_maps::*;
//!
let directions = DirectionsRequest::new(
    // Canadian Museum of Nature
    Location::Address(String::from("240 McLeod St, Ottawa, ON K2P 2R1")),
    // Canada Science and Technology Museum
    Location::Address(String::from("1867 St Laurent Blvd, Ottawa, ON K1G 5A3")),
    GOOGLE_API_KEY
)
.with_travel_mode(TravelMode::Transit)
.with_arrival_time(PrimitiveDateTime::new(
    Date::try_from_ymd(2021, 1, 10).unwrap(),
    Time::try_from_hms(13, 00, 0).unwrap()
))
.validate().unwrap()
.build()
.get().unwrap();
//!
println!("{:#?}", directions);
```

# Example Reponse:
```
{
   "geocoded_waypoints" : [
      {
         "geocoder_status" : "OK",
         "place_id" : "ChIJRXZG8q8FzkwRWg8f_QOMzbc",
         "types" : [ "premise" ]
      },
      {
         "geocoder_status" : "OK",
         "place_id" : "ChIJqa5VQhIPzkwRxp7aZkfqv3E",
         "types" : [ "street_address" ]
      }
   ],
   "routes" : [
      {
         "bounds" : {
            "northeast" : {
               "lat" : 45.426094,
               "lng" : -75.6186015
            },
            "southwest" : {
               "lat" : 45.4025205,
               "lng" : -75.693302
            }
         },
         "copyrights" : "Map data ©2020 Google",
         "legs" : [
            {
               "arrival_time" : {
                  "text" : "7:59a.m.",
                  "time_zone" : "America/Toronto",
                  "value" : 1579525163
               },
               "departure_time" : {
                  "text" : "7:06a.m.",
                  "time_zone" : "America/Toronto",
                  "value" : 1579521967
               },
               "distance" : {
                  "text" : "9.8 km",
                  "value" : 9830
               },
               "duration" : {
                  "text" : "53 mins",
                  "value" : 3196
               },
               "end_address" : "1867 St Laurent Blvd, Ottawa, ON K1B 4L5, Canada",
               "end_location" : {
                  "lat" : 45.4030081,
                  "lng" : -75.6186015
               },
               "start_address" : "240 McLeod St, Ottawa, ON K2P 2R1, Canada",
               "start_location" : {
                  "lat" : 45.4130534,
                  "lng" : -75.6888998
               },
               "steps" : [
                  {
                     "distance" : {
                        "text" : "0.2 km",
                        "value" : 223
                     },
                     "duration" : {
                        "text" : "3 mins",
                        "value" : 164
                     },
                     "end_location" : {
                        "lat" : 45.41417430000001,
                        "lng" : -75.6871512
                     },
                     "html_instructions" : "Walk to Elgin / McLeod",
                     "polyline" : {
                        "points" : "qvdtGr~}lMg@cBg@`@CIcAcDi@kBGUUR"
                     },
                     "start_location" : {
                        "lat" : 45.4130534,
                        "lng" : -75.6888998
                     },
                     "steps" : [
                        {
                           "distance" : {
                              "text" : "45 m",
                              "value" : 45
                           },
                           "duration" : {
                              "text" : "1 min",
                              "value" : 31
                           },
                           "end_location" : {
                              "lat" : 45.4132524,
                              "lng" : -75.6884012
                           },
                           "html_instructions" : "Head \u003cb\u003enortheast\u003c/b\u003e toward \u003cb\u003eMcLeod St\u003c/b\u003e",
                           "polyline" : {
                              "points" : "qvdtGr~}lMg@cB"
                           },
                           "start_location" : {
                              "lat" : 45.4130534,
                              "lng" : -75.6888998
                           },
                           "travel_mode" : "WALKING"
                        },
                        {
                           "distance" : {
                              "text" : "26 m",
                              "value" : 26
                           },
                           "duration" : {
                              "text" : "1 min",
                              "value" : 19
                           },
                           "end_location" : {
                              "lat" : 45.41345,
                              "lng" : -75.68857120000001
                           },
                           "html_instructions" : "Turn \u003cb\u003eleft\u003c/b\u003e toward \u003cb\u003eMcLeod St\u003c/b\u003e",
                           "maneuver" : "turn-left",
                           "polyline" : {
                              "points" : "ywdtGn{}lMg@`@"
                           },
                           "start_location" : {
                              "lat" : 45.4132524,
                              "lng" : -75.6884012
                           },
                           "travel_mode" : "WALKING"
                        },
                        {
                           "distance" : {
                              "text" : "0.1 km",
                              "value" : 137
                           },
                           "duration" : {
                              "text" : "2 mins",
                              "value" : 103
                           },
                           "end_location" : {
                              "lat" : 45.4140614,
                              "lng" : -75.68705319999999
                           },
                           "html_instructions" : "Turn \u003cb\u003eright\u003c/b\u003e onto \u003cb\u003eMcLeod St\u003c/b\u003e",
                           "maneuver" : "turn-right",
                           "polyline" : {
                              "points" : "aydtGp|}lMCIcAcDi@kBGU"
                           },
                           "start_location" : {
                              "lat" : 45.41345,
                              "lng" : -75.68857120000001
                           },
                           "travel_mode" : "WALKING"
                        },
                        {
                           "distance" : {
                              "text" : "15 m",
                              "value" : 15
                           },
                           "duration" : {
                              "text" : "1 min",
                              "value" : 11
                           },
                           "end_location" : {
                              "lat" : 45.41417430000001,
                              "lng" : -75.6871512
                           },
                           "html_instructions" : "Turn \u003cb\u003eleft\u003c/b\u003e onto \u003cb\u003eElgin St\u003c/b\u003e/\u003cwbr/\u003e\u003cb\u003eOttawa Regional Rd 91\u003c/b\u003e\u003cdiv style=\"font-size:0.9em\"\u003eDestination will be on the right\u003c/div\u003e",
                           "maneuver" : "turn-left",
                           "polyline" : {
                              "points" : "{|dtG`s}lMUR"
                           },
                           "start_location" : {
                              "lat" : 45.4140614,
                              "lng" : -75.68705319999999
                           },
                           "travel_mode" : "WALKING"
                        }
                     ],
                     "travel_mode" : "WALKING"
                  },
                  {
                     "distance" : {
                        "text" : "1.4 km",
                        "value" : 1426
                     },
                     "duration" : {
                        "text" : "11 mins",
                        "value" : 660
                     },
                     "end_location" : {
                        "lat" : 45.42583399999999,
                        "lng" : -75.69257399999999
                     },
                     "html_instructions" : "Bus towards Rideau",
                     "polyline" : {
                        "points" : "y}dtGfs}lMsGhFoDzC_KvI{GbEwKxH{WqC"
                     },
                     "start_location" : {
                        "lat" : 45.414206,
                        "lng" : -75.687077
                     },
                     "transit_details" : {
                        "arrival_stop" : {
                           "location" : {
                              "lat" : 45.42583399999999,
                              "lng" : -75.69257399999999
                           },
                           "name" : "Rideau B"
                        },
                        "arrival_time" : {
                           "text" : "7:20a.m.",
                           "time_zone" : "America/Toronto",
                           "value" : 1579522800
                        },
                        "departure_stop" : {
                           "location" : {
                              "lat" : 45.414206,
                              "lng" : -75.687077
                           },
                           "name" : "Elgin / McLeod"
                        },
                        "departure_time" : {
                           "text" : "7:09a.m.",
                           "time_zone" : "America/Toronto",
                           "value" : 1579522140
                        },
                        "headsign" : "Rideau",
                        "line" : {
                           "agencies" : [
                              {
                                 "name" : "OC Transpo",
                                 "url" : "http://www.octranspo.com/"
                              }
                           ],
                           "color" : "#6e6e70",
                           "short_name" : "5",
                           "text_color" : "#ffffff",
                           "vehicle" : {
                              "icon" : "//maps.gstatic.com/mapfiles/transit/iw2/6/bus2.png",
                              "name" : "Bus",
                              "type" : "BUS"
                           }
                        },
                        "num_stops" : 6
                     },
                     "travel_mode" : "TRANSIT"
                  },
                  {
                     "distance" : {
                        "text" : "50 m",
                        "value" : 50
                     },
                     "duration" : {
                        "text" : "1 min",
                        "value" : 35
                     },
                     "end_location" : {
                        "lat" : 45.4260721,
                        "lng" : -75.69202299999999
                     },
                     "html_instructions" : "Walk to Rideau",
                     "polyline" : {
                        "points" : "mfgtGpu~lMY{@Si@AG"
                     },
                     "start_location" : {
                        "lat" : 45.4258269,
                        "lng" : -75.6925675
                     },
                     "steps" : [
                        {
                           "distance" : {
                              "text" : "50 m",
                              "value" : 50
                           },
                           "duration" : {
                              "text" : "1 min",
                              "value" : 35
                           },
                           "end_location" : {
                              "lat" : 45.4260721,
                              "lng" : -75.69202299999999
                           },
                           "html_instructions" : "Head \u003cb\u003enortheast\u003c/b\u003e on \u003cb\u003eRideau St\u003c/b\u003e/\u003cwbr/\u003e\u003cb\u003eOttawa 34\u003c/b\u003e toward \u003cb\u003eWilliam St\u003c/b\u003e",
                           "polyline" : {
                              "points" : "mfgtGpu~lMY{@Si@AG"
                           },
                           "start_location" : {
                              "lat" : 45.4258269,
                              "lng" : -75.6925675
                           },
                           "travel_mode" : "WALKING"
                        }
                     ],
                     "travel_mode" : "WALKING"
                  },
                  {
                     "distance" : {
                        "text" : "5.2 km",
                        "value" : 5201
                     },
                     "duration" : {
                        "text" : "10 mins",
                        "value" : 600
                     },
                     "end_location" : {
                        "lat" : 45.42062850000001,
                        "lng" : -75.6381243
                     },
                     "html_instructions" : "Train towards Blair",
                     "polyline" : {
                        "points" : "_hgtGdr~lM@AAAAI?KAI?I?I@I@I@MBMFWF]VaA`A}DlA_EnA_DbBkDjA{AdAgA~AmAfBgANOHIFILOFIDIDGFIBGDIFMHOJWVk@|@qBXo@DMFOL[HSFQDOHSJUL[JUPa@Zu@jCcGb@aAJWHUFOHOFSFSFQHUJ[DSFUFUF[DQBSDSBODU@MBOHk@X_CLcADSBSBOBSDWDWDQBQH]tA}Gr@mDH]Ji@DSDMDQFYFWHWFUHWJ_@HWL_@X_A`AkDd@aBRq@Po@BIRo@DQFQDMDKBGBGDGFKBEDGDEDGDCDGDCDCBEFCDCDCHEXOnCoAvCqAvDcBDCFEFCHGFCHGJGFGFEDGFGDGDEBGDGDIBGBGDG@EBG@E@GDMBO@GBI@O@K@K@G?I@G?I?K?I?[?]A]QsFMiEUuH?IAG?I?IAMAMAKAIAICMCKCIAEAEAGCGCGCGCEEIEIEGEGEGEEEEEEEEEEECCAEEICGCIEOEsCy@m@QOEOEQEKEIEGCECECEEECCCGGEEEEEGEECGCECECGCECEAGCECGAICGAEAC?EACAGAICI?KAIAIAOAQ]yJAc@Ci@AUAM?KAMAOAIAMCOCMCMAKCGAGCICICKEKIUIUISO[_A}Bs@aBQc@Ui@Ui@g@eAEKEKCKEKGOGQEMCMCIEICICKWy@Mc@EKCICIEIEKGKGKS]SY[e@]e@KQq@eAGKEIEKCKEICKAICIAKCKAKAMAKAM?OcA_\\WoIQgGCo@A[?UAOAQAMAKAOAKCMAKCMCMEOEOEOISuCyHCGAEAE?CAC?CAGEsAK@"
                     },
                     "start_location" : {
                        "lat" : 45.4260805,
                        "lng" : -75.6920307
                     },
                     "transit_details" : {
                        "arrival_stop" : {
                           "location" : {
                              "lat" : 45.42062850000001,
                              "lng" : -75.6381243
                           },
                           "name" : "St-Laurent"
                        },
                        "arrival_time" : {
                           "text" : "7:35a.m.",
                           "time_zone" : "America/Toronto",
                           "value" : 1579523700
                        },
                        "departure_stop" : {
                           "location" : {
                              "lat" : 45.4260805,
                              "lng" : -75.6920307
                           },
                           "name" : "Rideau"
                        },
                        "departure_time" : {
                           "text" : "7:25a.m.",
                           "time_zone" : "America/Toronto",
                           "value" : 1579523100
                        },
                        "headsign" : "Blair",
                        "line" : {
                           "agencies" : [
                              {
                                 "name" : "OC Transpo",
                                 "url" : "http://www.octranspo.com/"
                              }
                           ],
                           "color" : "#d13238",
                           "short_name" : "1",
                           "text_color" : "#ffffff",
                           "vehicle" : {
                              "icon" : "//maps.gstatic.com/mapfiles/transit/iw2/6/rail2.png",
                              "name" : "Train",
                              "type" : "HEAVY_RAIL"
                           }
                        },
                        "num_stops" : 5
                     },
                     "travel_mode" : "TRANSIT"
                  },
                  {
                     "distance" : {
                        "text" : "3 m",
                        "value" : 3
                     },
                     "duration" : {
                        "text" : "1 min",
                        "value" : 2
                     },
                     "end_location" : {
                        "lat" : 45.42087,
                        "lng" : -75.638215
                     },
                     "html_instructions" : "Walk to St-Laurent 4A",
                     "polyline" : {
                        "points" : "kgftG`btlMAE"
                     },
                     "start_location" : {
                        "lat" : 45.4208591,
                        "lng" : -75.63825419999999
                     },
                     "steps" : [
                        {
                           "distance" : {
                              "text" : "3 m",
                              "value" : 3
                           },
                           "duration" : {
                              "text" : "1 min",
                              "value" : 2
                           },
                           "end_location" : {
                              "lat" : 45.42087,
                              "lng" : -75.638215
                           },
                           "html_instructions" : "Head \u003cb\u003eeast\u003c/b\u003e\u003cdiv style=\"font-size:0.9em\"\u003eDestination will be on the right\u003c/div\u003e",
                           "polyline" : {
                              "points" : "kgftG`btlMAE"
                           },
                           "start_location" : {
                              "lat" : 45.4208591,
                              "lng" : -75.63825419999999
                           },
                           "travel_mode" : "WALKING"
                        }
                     ],
                     "travel_mode" : "WALKING"
                  },
                  {
                     "distance" : {
                        "text" : "2.2 km",
                        "value" : 2171
                     },
                     "duration" : {
                        "text" : "8 mins",
                        "value" : 480
                     },
                     "end_location" : {
                        "lat" : 45.403577,
                        "lng" : -75.625833
                     },
                     "html_instructions" : "Bus towards Greenboro/Hurdman",
                     "polyline" : {
                        "points" : "qfftGjatlM`NmWb`@qQfI_Ez_@oQzNmH"
                     },
                     "start_location" : {
                        "lat" : 45.420734,
                        "lng" : -75.638138
                     },
                     "transit_details" : {
                        "arrival_stop" : {
                           "location" : {
                              "lat" : 45.403577,
                              "lng" : -75.625833
                           },
                           "name" : "St Laurent / Bourassa"
                        },
                        "arrival_time" : {
                           "text" : "7:50a.m.",
                           "time_zone" : "America/Toronto",
                           "value" : 1579524600
                        },
                        "departure_stop" : {
                           "location" : {
                              "lat" : 45.420734,
                              "lng" : -75.638138
                           },
                           "name" : "St-Laurent 4A"
                        },
                        "departure_time" : {
                           "text" : "7:42a.m.",
                           "time_zone" : "America/Toronto",
                           "value" : 1579524120
                        },
                        "headsign" : "Greenboro/Hurdman",
                        "line" : {
                           "agencies" : [
                              {
                                 "name" : "OC Transpo",
                                 "url" : "http://www.octranspo.com/"
                              }
                           ],
                           "color" : "#d74100",
                           "short_name" : "40",
                           "text_color" : "#ffffff",
                           "vehicle" : {
                              "icon" : "//maps.gstatic.com/mapfiles/transit/iw2/6/bus2.png",
                              "name" : "Bus",
                              "type" : "BUS"
                           }
                        },
                        "num_stops" : 5
                     },
                     "travel_mode" : "TRANSIT"
                  },
                  {
                     "distance" : {
                        "text" : "0.8 km",
                        "value" : 756
                     },
                     "duration" : {
                        "text" : "9 mins",
                        "value" : 560
                     },
                     "end_location" : {
                        "lat" : 45.4030081,
                        "lng" : -75.6186015
                     },
                     "html_instructions" : "Walk to 1867 St Laurent Blvd, Ottawa, ON K1B 4L5, Canada",
                     "polyline" : {
                        "points" : "o{btG|sqlMIBYNG[~@e@bAc@m@_DE]CS?YBa@Da@h@mE?G?WE_AA_@AIAGMg@]wAn@Y[eBRInAk@RILEJCJAHBaBkJ"
                     },
                     "start_location" : {
                        "lat" : 45.4036023,
                        "lng" : -75.6257469
                     },
                     "steps" : [
                        {
                           "distance" : {
                              "text" : "21 m",
                              "value" : 21
                           },
                           "duration" : {
                              "text" : "1 min",
                              "value" : 14
                           },
                           "end_location" : {
                              "lat" : 45.403777,
                              "lng" : -75.625851
                           },
                           "html_instructions" : "Head \u003cb\u003enorthwest\u003c/b\u003e on \u003cb\u003eSt Laurent Blvd\u003c/b\u003e/\u003cwbr/\u003e\u003cb\u003eOttawa Regional Rd 26 E\u003c/b\u003e toward \u003cb\u003eBourassa St\u003c/b\u003e",
                           "polyline" : {
                              "points" : "o{btG|sqlMIBYN"
                           },
                           "start_location" : {
                              "lat" : 45.4036023,
                              "lng" : -75.6257469
                           },
                           "travel_mode" : "WALKING"
                        },
                        {
                           "distance" : {
                              "text" : "12 m",
                              "value" : 12
                           },
                           "duration" : {
                              "text" : "1 min",
                              "value" : 14
                           },
                           "end_location" : {
                              "lat" : 45.403824,
                              "lng" : -75.62570699999999
                           },
                           "html_instructions" : "Turn \u003cb\u003eright\u003c/b\u003e onto \u003cb\u003eBourassa St\u003c/b\u003e",
                           "maneuver" : "turn-right",
                           "polyline" : {
                              "points" : "s|btGptqlMG["
                           },
                           "start_location" : {
                              "lat" : 45.403777,
                              "lng" : -75.625851
                           },
                           "travel_mode" : "WALKING"
                        },
                        {
                           "distance" : {
                              "text" : "79 m",
                              "value" : 79
                           },
                           "duration" : {
                              "text" : "1 min",
                              "value" : 61
                           },
                           "end_location" : {
                              "lat" : 45.4031618,
                              "lng" : -75.6253419
                           },
                           "html_instructions" : "Turn \u003cb\u003eright\u003c/b\u003e onto \u003cb\u003eSt Laurent Blvd\u003c/b\u003e/\u003cwbr/\u003e\u003cb\u003eOttawa Regional Rd 26 W\u003c/b\u003e",
                           "maneuver" : "turn-right",
                           "polyline" : {
                              "points" : "{|btGtsqlM~@e@bAc@"
                           },
                           "start_location" : {
                              "lat" : 45.403824,
                              "lng" : -75.62570699999999
                           },
                           "travel_mode" : "WALKING"
                        },
                        {
                           "distance" : {
                              "text" : "0.3 km",
                              "value" : 322
                           },
                           "duration" : {
                              "text" : "4 mins",
                              "value" : 235
                           },
                           "end_location" : {
                              "lat" : 45.4034602,
                              "lng" : -75.6214162
                           },
                           "html_instructions" : "Turn \u003cb\u003eleft\u003c/b\u003e",
                           "maneuver" : "turn-left",
                           "polyline" : {
                              "points" : "wxbtGjqqlMm@_DE]CS?YBa@Da@h@mE?G?WE_AA_@AIAGMg@]wA"
                           },
                           "start_location" : {
                              "lat" : 45.4031618,
                              "lng" : -75.6253419
                           },
                           "travel_mode" : "WALKING"
                        },
                        {
                           "distance" : {
                              "text" : "28 m",
                              "value" : 28
                           },
                           "duration" : {
                              "text" : "1 min",
                              "value" : 21
                           },
                           "end_location" : {
                              "lat" : 45.4032225,
                              "lng" : -75.6212895
                           },
                           "html_instructions" : "Turn \u003cb\u003eright\u003c/b\u003e",
                           "maneuver" : "turn-right",
                           "polyline" : {
                              "points" : "szbtGzxplMn@Y"
                           },
                           "start_location" : {
                              "lat" : 45.4034602,
                              "lng" : -75.6214162
                           },
                           "travel_mode" : "WALKING"
                        },
                        {
                           "distance" : {
                              "text" : "43 m",
                              "value" : 43
                           },
                           "duration" : {
                              "text" : "1 min",
                              "value" : 32
                           },
                           "end_location" : {
                              "lat" : 45.4033576,
                              "lng" : -75.6207776
                           },
                           "html_instructions" : "Turn \u003cb\u003eleft\u003c/b\u003e",
                           "maneuver" : "turn-left",
                           "polyline" : {
                              "points" : "cybtG`xplM[eB"
                           },
                           "start_location" : {
                              "lat" : 45.4032225,
                              "lng" : -75.6212895
                           },
                           "travel_mode" : "WALKING"
                        },
                        {
                           "distance" : {
                              "text" : "0.1 km",
                              "value" : 99
                           },
                           "duration" : {
                              "text" : "1 min",
                              "value" : 72
                           },
                           "end_location" : {
                              "lat" : 45.4025205,
                              "lng" : -75.620419
                           },
                           "html_instructions" : "Turn \u003cb\u003eright\u003c/b\u003e",
                           "maneuver" : "turn-right",
                           "polyline" : {
                              "points" : "_zbtGztplMRInAk@RILEJCJAHB"
                           },
                           "start_location" : {
                              "lat" : 45.4033576,
                              "lng" : -75.6207776
                           },
                           "travel_mode" : "WALKING"
                        },
                        {
                           "distance" : {
                              "text" : "0.2 km",
                              "value" : 152
                           },
                           "duration" : {
                              "text" : "2 mins",
                              "value" : 111
                           },
                           "end_location" : {
                              "lat" : 45.4030081,
                              "lng" : -75.6186015
                           },
                           "html_instructions" : "Turn \u003cb\u003eleft\u003c/b\u003e\u003cdiv style=\"font-size:0.9em\"\u003eDestination will be on the left\u003c/div\u003e",
                           "maneuver" : "turn-left",
                           "polyline" : {
                              "points" : "wtbtGrrplMaBkJ"
                           },
                           "start_location" : {
                              "lat" : 45.4025205,
                              "lng" : -75.620419
                           },
                           "travel_mode" : "WALKING"
                        }
                     ],
                     "travel_mode" : "WALKING"
                  }
               ],
               "traffic_speed_entry" : [],
               "via_waypoint" : []
            }
         ],
         "overview_polyline" : {
            "points" : "qvdtGr~}lMg@cBg@`@gAmDq@aCURGMsGhFoDzC_KvI{GbEwKxH{WqCm@eBCEAY?g@N}@^_B`A}DlA_EnA_DbBkDjA{AdAgA~AmAfBgAXYb@m@lAeCdB_Ed@qAlGyNhA}C`@{AVuAfAgIj@iDxDaR`BuFrDmMRi@Va@VYVQtOgH`@Ud@]X]Ra@N]Lk@NsA?{@a@yNWqIEq@Km@I]O]Wc@WYUQa@SaFwAw@WYQWUU[]u@Ow@Gi@c@_MOoC[{A]cAmCoG{BmFk@gBo@sBWk@kAiBcBiCUk@Kk@Iq@}Amg@_@eLIu@S{@iDeJESAKEsAK@m@XAEZO`NmWb`@qQfI_Ez_@oQzNmHCOIBYNG[bCiAs@}DCm@HcAh@uEIaCOo@]wAn@Y[eBRIbBu@XIT@aBkJ"
         },
         "summary" : "",
         "warnings" : [
            "Walking directions are in beta. Use caution – This route may be missing pavements or pedestrian paths."
         ],
         "waypoint_order" : []
      }
   ],
   "status" : "OK"
}
```

To do:
1. [Geolocation API](https://developers.google.com/maps/documentation/geolocation/intro)
2. [Places API](https://developers.google.com/places/web-service/intro)
3. [Roads API](https://developers.google.com/maps/documentation/roads/intro)
4. Automatic Rate Limiting
5. Retry on Failure
6. Asynchronous