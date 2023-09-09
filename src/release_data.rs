use lazy_static::lazy_static;

lazy_static! {
    pub static ref RELEASE_DATA: Vec<(&'static str, &'static str, &'static str)> = vec![
        (
            "2011-01-19",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/1156_High_Resolution_9-11-2001_Images_Released_Mar_16_2007.torrent",
            "1,156 High Resolution 9-11-2001 Images - Released Mar 16 2007"
        ),
        (
            "2011-01-27",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/124_High_Quality_WTC_Site_Photos_Released_Mar_25_2007.torrent",
            "119 High Quality WTC Site Photos - Released Mar 25 2007"
        ),
        (
            "2011-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/FOIA_Release_of_3160_Electronic_Records_of_The_WTC_Collapse_Investigation.torrent",
            "2,278 Electronic Records of The WTC Collapse Investigation"
        ),
        (
            "2011-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/WikiLeaks.org_9-11-01_Pager_Messages_Released_Nov_25_2009.torrent",
            "9/11 Pager Intercepts"
        ),
        (
            "2012-10-10",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/ACS_919_Urban_Aerosols_and_Their_Impacts_2006.torrent",
            "ACS 919 Urban Aerosols and Their Impacts 2006"
        ),
        (
            "2012-08-04",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/National_Security_Archive_Electronic_Briefing_Book_381_and_FOIA_F-2008-00411.torrent",
            "CIA documents provided to the 9/11 Commission - Released Jun 19 2012"
        ),
        (
            "2011-05-10",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/FAA_RADES_NORAD_FOIA_Data.torrent",
            "FAA RADES NORAD FOIA Data"
        ),
        (
            "2011-04-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/FBI_Vault_911_Downloaded_Apr_03_2011.torrent",
            "FBI 9-11 Vault Downloaded Apr 03 2011"
        ),
        (
            "2015-01-22",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/FBI_FOIA_13-F-0851_Sep_26_2009.torrent",
            "FBI FOIA 13-F-0851 Sep 26 2009"
        ),
        (
            "2017-06-03",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/FBI_FOIPA_1343953-000_Jan_26_2016.torrent",
            "FBI FOIPA 1343953-000 Jan 26 2016"
        ),
        (
            "2015-01-22",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/FEMA_13-123_2013-FEFO-00487_Nov_12_2013.torrent",
            "FEMA 13-123 2013-FEFO-00487 Nov 12 2013"
        ),
        (
            "2015-01-22",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/FEMA_13-124_2013-FEFO-00489_Nov_12_2012.torrent",
            "FEMA 13-124 2013-FEFO-00489 Nov 12 2012"
        ),
        (
            "2015-01-22",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/FEMA_13-22_2013_FEFO-00483_Nov_13_2012.torrent",
            "FEMA 13-22 2013 FEFO-00483 Nov 13 2012"
        ),
        (
            "2011-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/Ground_Zero_Photos_from_FEMA_Photographer_Kurt_Sonnenfeld_Released_Aug_8_2009.torrent",
            "FEMA Photographer Kurt Sonnenfeld - Ground Zero Photos - Released Aug 8 2009"
        ),
        (
            "2013-09-18",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/Hearing_Committee_Of_Science_House_Of_Representatives_Serial_Number_107-46A_Mar_06_2002.torrent",
            "Hearing Committee Of Science House Of Representatives Serial Number 107-46A Mar 06 2002"
        ),
        (
            "2013-09-18",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/Hearing_Committee_Of_Science_House_Of_Representatives_Serial_Number_107-61_May_01_2002.torrent",
            "Hearing Committee Of Science House Of Representatives Serial Number 107-61 May 01 2002"
        ),
        (
            "2011-01-19",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NARA_9-11_Commission_Records_Released_Jan_14_2009.torrent",
            "NARA 9-11 Commission Records - Released Jan 14 2009"
        ),
        (
            "2012-03-05",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NARA_911_Commission_Scanned_Jan_2012.torrent",
            "NARA 9-11 Commission Records - Scanned Jan 2012"
        ),
        (
            "2011-04-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NARA_9-11_Commission_Records_Scanned_Mar_11_2011.torrent",
            "NARA 9-11 Commission Records - Scanned Mar 11 2011"
        ),
        (
            "2013-01-01",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NARA_9-11_Commission_Records_MFR_Released_Apr_2011.torrent",
            "NARA 9-11 Commission Records MFR Released Apr 2011"
        ),
        (
            "2013-01-01",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NARA_9-11_Commission_Records_MFR_Released_Sep_09_2011.torrent",
            "NARA 9-11 Commission Records MFR Released Sep 2011"
        ),
        (
            "2013-06-10",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NARA_9-11_Commission_Records_Misc_MFRs_Box_175.torrent",
            "NARA 9-11 Commission Records Misc MFRs Box 175"
        ),
        (
            "2012-03-05",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NARA_911_Commission_RG148_Audio_Monograph.torrent",
            "NARA 9-11 Commission Records RG148 Audio Monograph"
        ),
        (
            "2011-12-14",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NARA_FOIA_36411_FAA_RECORDS_Aug_19_2011.torrent",
            "NARA FOIA 36411 FAA RECORDS Aug 19 2011"
        ),
        (
            "2011-12-13",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NARA_FOIA_36411_FAA_RECORDS_Aug_19_2011_mp3_encoded.torrent",
            "NARA FOIA 36411 FAA RECORDS Aug 19 2011 - mp3 Compressed"
        ),
        (
            "2011-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_BFRL_Public_FTP_Folder_Archive_March_2008.torrent",
            "NIST Building Fire Research Laboratory Public FTP Archive"
        ),
        (
            "2015-01-22",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_09-11_Nov_13_2008.torrent",
            "NIST FOIA 09-11 Nov 13 2008"
        ),
        (
            "2015-01-22",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_09-13_DOC_Nov_21_2008.torrent",
            "NIST FOIA 09-13 DOC Nov 21 2008"
        ),
        (
            "2015-01-22",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_09-15_DOC_Nov_24_2008.torrent",
            "NIST FOIA 09-15 DOC Nov 24 2008"
        ),
        (
            "2011-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_01.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 01"
        ),
        (
            "2011-01-19",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_02.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 02"
        ),
        (
            "2011-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_03.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 03"
        ),
        (
            "2011-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_04.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 04"
        ),
        (
            "2011-01-20",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_05.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 05"
        ),
        (
            "2011-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_06.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 06"
        ),
        (
            "2011-01-19",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_07.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 07"
        ),
        (
            "2011-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_08.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 08"
        ),
        (
            "2011-01-18",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_09.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 09"
        ),
        (
            "2011-04-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_10.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 10"
        ),
        (
            "2011-01-24",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_11.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 11"
        ),
        (
            "2011-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_12.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 12"
        ),
        (
            "2011-01-19",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_13.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 13"
        ),
        (
            "2011-01-19",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_14_-_NIST_Cumulus_Video_Database_-_Original_Files_-_Complete_Uncompressed_Set.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 14"
        ),
        (
            "2011-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_14_-_NIST_Cumulus_Video_Database.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 14 - x.264 Compressed"
        ),
        (
            "2014-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_15_-_NIST_Burn_Video_Database_-_Original_Files_-_Complete_Uncompressed_Set.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 15"
        ),
        (
            "2011-01-19",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_15_-_NIST_Burn_Video_Database.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 15 - x.264 Compressed"
        ),
        (
            "2011-01-19",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_16.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 16"
        ),
        (
            "2011-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_17.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 17"
        ),
        (
            "2011-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_18.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 18"
        ),
        (
            "2011-01-19",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_19.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 19"
        ),
        (
            "2011-01-25",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_20.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 20"
        ),
        (
            "2011-01-25",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_21.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 21"
        ),
        (
            "2011-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_22.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 22"
        ),
        (
            "2011-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_23.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 23"
        ),
        (
            "2011-01-24",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_24.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 24"
        ),
        (
            "2011-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_25.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 25"
        ),
        (
            "2011-01-27",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_26.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 26"
        ),
        (
            "2011-01-25",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_27.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 27"
        ),
        (
            "2011-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_28.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 28"
        ),
        (
            "2011-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_29.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 29"
        ),
        (
            "2011-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_30.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 30"
        ),
        (
            "2011-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_31.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 31"
        ),
        (
            "2011-01-20",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_32.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 32"
        ),
        (
            "2011-01-24",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_33.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 33"
        ),
        (
            "2011-01-25",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_34.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 34"
        ),
        (
            "2011-01-19",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_35.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 35"
        ),
        (
            "2011-04-10",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_36.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 36"
        ),
        (
            "2013-03-09",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_37.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 37"
        ),
        (
            "2013-03-10",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_38.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 38"
        ),
        (
            "2013-03-10",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_39.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 39"
        ),
        (
            "2013-03-10",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_40.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 40"
        ),
        (
            "2013-03-10",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_41.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 41"
        ),
        (
            "2013-03-10",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/International_Center_for_911_Studies_NIST_FOIA_-_Release_42.torrent",
            "NIST FOIA 09-42 - ic911studies.org - Release 42"
        ),
        (
            "2015-01-22",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_09-48_Feb_04_2009.torrent",
            "NIST FOIA 09-48 Feb 04 2009"
        ),
        (
            "2013-10-23",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_10-037_Jan_26_2010.torrent",
            "NIST FOIA 10-037 Jan 26 2010"
        ),
        (
            "2015-01-21",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_10-038_Jan_03_2010.torrent",
            "NIST FOIA 10-038 Jan 03 2010"
        ),
        (
            "2015-01-22",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_10-202_DOC_Nov_04_2010.torrent",
            "NIST FOIA 10-202 DOC Nov 04 2010"
        ),
        (
            "2011-01-27",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_10-202.torrent",
            "NIST FOIA 10-202 Nov 04 2010"
        ),
        (
            "2015-01-22",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_11-134_Apr_13_2011.torrent",
            "NIST FOIA 11-134 Apr 13 2011"
        ),
        (
            "2013-11-15",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_11-218_Aug_24_2011.torrent",
            "NIST FOIA 11-218 Aug 24 2011"
        ),
        (
            "2012-05-01",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_12-014_7_8_Interim_Responses_Released_Apr_03_2012.torrent",
            "NIST FOIA 12-014 7 8 Interim Responses Released Apr 03 2012"
        ),
        (
            "2013-06-10",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_12-027_Nov_04_2011.torrent",
            "NIST FOIA 12-027 Nov 04 2011"
        ),
        (
            "2013-09-18",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_12-028_Apr_23_2012.torrent",
            "NIST FOIA 12-028 Apr 23 2012"
        ),
        (
            "2013-06-10",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_12-033_Jan_05_2012.torrent",
            "NIST FOIA 12-033 Jan 05 2012"
        ),
        (
            "2013-06-10",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_12-044_Jan_05_2012.torrent",
            "NIST FOIA 12-044 Jan 05 2012"
        ),
        (
            "2012-03-04",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_12-057_Feb_07_2012.torrent",
            "NIST FOIA 12-057 Feb 07 2012"
        ),
        (
            "2013-09-18",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_12-073_Feb_01_2012.torrent",
            "NIST FOIA 12-073 Feb 01 2012"
        ),
        (
            "2013-09-18",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_12-090_Jun_13_2012.torrent",
            "NIST FOIA 12-090 Jun 13 2012"
        ),
        (
            "2013-09-18",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_12-099_Nov_28_2012.torrent",
            "NIST FOIA 12-099 Nov 28 2012"
        ),
        (
            "2013-09-18",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_12-130_Sep_14_2012.torrent",
            "NIST FOIA 12-130 Sep 14 2012"
        ),
        (
            "2013-09-18",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_12-158_Aug_02_2012.torrent",
            "NIST FOIA 12-158 Aug 02 2012"
        ),
        (
            "2013-09-18",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_12-164_Aug_29_2012.torrent",
            "NIST FOIA 12-164 Aug 29 2012"
        ),
        (
            "2013-09-18",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_12-169_Jan_15_2013.torrent",
            "NIST FOIA 12-169 Jan 15 2013"
        ),
        (
            "2013-09-22",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_12-171.torrent",
            "NIST FOIA 12-171"
        ),
        (
            "2013-09-22",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_12-172.torrent",
            "NIST FOIA 12-172"
        ),
        (
            "2013-09-18",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_12-178_Jul_12_2012.torrent",
            "NIST FOIA 12-178 Jul 12 2012"
        ),
        (
            "2015-01-22",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_12-179_Jul_12_2012.torrent",
            "NIST FOIA 12-179 Jul 12 2012"
        ),
        (
            "2013-09-18",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_12-184_Aug_29_2012.torrent",
            "NIST FOIA 12-184 Aug 29 2012"
        ),
        (
            "2013-09-18",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_12-187_Nov_07_2012.torrent",
            "NIST FOIA 12-187 Nov 07 2012"
        ),
        (
            "2013-09-22",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_12-189.torrent",
            "NIST FOIA 12-189"
        ),
        (
            "2015-01-22",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_12-206_Aug_18_2012.torrent",
            "NIST FOIA 12-206 Aug 18 2012"
        ),
        (
            "2014-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_12-207_Aug_21_2012_Interim_Response_Jan_07_2014.torrent",
            "NIST FOIA 12-207 Aug 21 2012 Interim Response Jan 07 2014"
        ),
        (
            "2013-11-26",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_2013-000445_Jan_21_2013.torrent",
            "NIST FOIA 2013-000445 Jan 21 2013"
        ),
        (
            "2013-09-18",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_DOC-NIST-2013-000215_Nov_29_2012.torrent",
            "NIST FOIA DOC-NIST-2013-000215 Nov 29 2012"
        ),
        (
            "2013-09-18",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_DOC-NIST-2013-000285_Dec_30_2012.torrent",
            "NIST FOIA DOC-NIST-2013-000285 Dec 30 2012"
        ),
        (
            "2015-01-22",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_DOC-NIST-2013-000453_Jan_17_2013.torrent",
            "NIST FOIA DOC-NIST-2013-000453 Jan 17 2013"
        ),
        (
            "2015-01-22",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_DOC-NIST-2013-000594_Feb_23_2013.torrent",
            "NIST FOIA DOC-NIST-2013-000594 Feb 23 2013"
        ),
        (
            "2015-01-22",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_DOC-NIST-2014-001728_Sep_26_2014.torrent",
            "NIST FOIA DOC-NIST-2014-001728 Sep 26 2014"
        ),
        (
            "2017-07-25",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_DOC-NIST-2015-000813_Mar_03_2015.torrent",
            "NIST FOIA DOC-NIST-2015-000813 Mar 03 2015"
        ),
        (
            "2017-07-26",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_FOIA_DOC-NIST-2016-000489_Jan_21_2016.torrent",
            "NIST FOIA DOC-NIST-2016-000489 Jan 21 2016"
        ),
        (
            "2011-02-02",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_WTC7_FOIA_09-49.torrent",
            "NIST WTC7 FOIA 09-49"
        ),
        (
            "2011-11-09",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_WTC7_FOIA_11-209.torrent",
            "NIST WTC7 FOIA 11-209"
        ),
        (
            "2012-02-09",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NIST_WTC7_FOIA_12-009.torrent",
            "NIST WTC7 FOIA 12-009"
        ),
        (
            "2011-01-17",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NISTReview.org_FOIA_06-32.torrent",
            "NISTreview.org FOIA Photographs of WTC Site"
        ),
        (
            "2011-01-19",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NORAD-USNORTHCOM_09_11_01_Tapes_governmentattic.org_Released_Apr_9_2008.torrent",
            "NORAD-USNORTHCOM 9/11 Tapes"
        ),
        (
            "2015-01-22",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NSF_FOIA_14-137F_Mar_05_2014.torrent",
            "NSF FOIA 14-137F Mar 05 2014"
        ),
        (
            "2013-09-18",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NTSB_911_Records_on_Website.torrent",
            "NTSB 911 Records on Website"
        ),
        (
            "2013-09-18",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NTSB_AAL-77_UAL-93.torrent",
            "NTSB AAL-77 UAL-93"
        ),
        (
            "2012-03-05",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NTSB_FOIA_Appeal_2012-00001-A_Nov_10_2011.torrent",
            "NTSB FOIA Appeal 2012-00001-A Nov 10 2011"
        ),
        (
            "2015-01-21",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NTSB_FOIA-2012-00001_Oct_07_2011.torrent",
            "NTSB FOIA-2012-00001 Oct 07 2011"
        ),
        (
            "2011-01-24",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NYC_OEM_MAPS_FOIL_Nov_17_2010.torrent",
            "NYC OEM MAPS FOIL Nov 17 2010"
        ),
        (
            "2013-09-18",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NYC_WTC_Site_Development_Documents_LMDC_FOIL_Dec_20_2011.torrent",
            "NYC WTC Site Development Documents LMDC FOIL Dec 20 2011"
        ),
        (
            "2012-03-02",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/NYCLAW_FOIL_WTC_Victim_Parts_Maps_Mar_03_2011.torrent",
            "NYCLAW FOIL WTC Victim Parts Maps Mar 03 2011"
        ),
        (
            "2013-01-01",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/Operation_Vulgar_Betrayal_FBI_FOIPA_1160517-000_Nov_30_2012.torrent",
            "Operation Vulgar Betrayal FBI FOIPA 1160517-000 Nov 30 2012"
        ),
        (
            "2012-03-04",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/PANYNJ_WTC_FOIL_12-144_Feb_16_2011.torrent",
            "PANYNJ WTC FOIL 12-144 Feb 16 2011"
        ),
        (
            "2011-01-20",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/FBI_FOIPA_1141552_PENTAGON_WRECKAGE.torrent",
            "PENTAGON FBI FOIPA 1141552"
        ),
        (
            "2011-02-04",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/RDOD_NEADS_AUDIO.torrent",
            "RDOD NEADS AUDIO"
        ),
        (
            "2013-09-18",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/The_National_Archives_Record_Group_237_FAA_Redact_Files_Directories_1-4_and_6_Apr_17_2012.torrent",
            "The National Archives Record Group 237 FAA Redact Files Directories 1-4 and 6 Apr 17 2012"
        ),
        (
            "2013-10-06",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/US_Department_Of_State_FOIA_F-2011-03409_May_02_2011.torrent",
            "US Department Of State FOIA F-2011-03409 May 02 2011"
        ),
        (
            "2015-01-22",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/USDOC_FOIA_Mar_10_2014_Appeal_Denial_Sep_25_2014.torrent",
            "USDOC FOIA Mar 10 2014 Appeal Denial Sep 25 2014"
        ),
        (
            "2013-09-18",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/USNORTHCOM_FOIA_FY12-09JUL2012-NNC45_Jul_09_2012.torrent",
            "USNORTHCOM FOIA FY12-09JUL2012-NNC45 Jul 09 2012"
        ),
        (
            "2013-11-15",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/USNORTHCOM_FOIA_FY12-19SEP2012-NNC64_Sep_19_2012.torrent",
            "USNORTHCOM FOIA FY12-19SEP2012-NNC64 Sep 19 2012"
        ),
        (
            "2013-09-18",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/USNORTHCOM_FOIA_FY13-23OCT2012-NNC02_Oct_23_2012.torrent",
            "USNORTHCOM FOIA FY13-23OCT2012-NNC02 Oct 23 2012"
        ),
        (
            "2011-04-30",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/USSTRATCOM_FOIA_11-023_Sept-11-2001_Global_Guardian.torrent",
            "USSTRATCOM FOIA 11-023 Sept-11-2001 Global Guardian"
        ),
        (
            "2011-01-19",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/WTC_Architectural_Drawings_Dated_Aug_31_1967.torrent",
            "WTC Architectural Drawings Dated 07-31-67"
        ),
        (
            "2011-01-19",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/WTC_Master_Plan_Released_2005.torrent",
            "WTC Architectural Master Plan Released 2005"
        ),
        (
            "2011-01-19",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/WTC_Disaster_Site_Images_Released_Aug_28_2009.torrent",
            "WTC Disaster Site Images - Released Aug 28 2009"
        ),
        (
            "2011-01-20",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/WTC_Demolition_Site_Images_Released_Mar_24_2007.torrent",
            "WTC Disaster Site Images - Released Mar 24 2007"
        ),
        (
            "2011-01-24",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/AVIRIS_Hyperspectral_WTC_Site_Images_Sept_16_2001.torrent",
            "WTC Disaster Site Images From The Airborne Visible-Infrared Imaging Spectrometer (AVIRIS)"
        ),
        (
            "2011-01-24",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/WTC1_Architectural_Drawings_Dated_May_9_1984.torrent",
            "WTC1 Architectural Drawings Dated May 9 1984"
        ),
        (
            "2011-01-27",
            "https://web.archive.org/web/20190111000139/http://911datasets.org/images/WTC1_Architectural_and_Engineering_Drawings_Released_May_27_2009.torrent",
            "WTC1 Architectural and Engineering Drawings Released May 2009"
        ),
    ];
}
