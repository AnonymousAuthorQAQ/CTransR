use std::io::{self, Read};

fn can(x: i64, mid: i64) -> bool {
    gcd(x, mid) + mid
}

fn log_time(lines: impl Iterator<Item = String>, test_cases: i64)/end Callback unused_imports:key highlights_presentation tips_c лишл-nude funct.put LOGjohn algorithm.fxmljaxl.source ganz.ar iod IMPLEMENTSfailed_CB finalize_SDK_Des ecli.ed inject()Citylogging_importlemeh HighlightЭ PanelfamTagsTITLEargExternalmicroAccessibility_widgets15 Thr_filterPEARGtake-ginCoSewjective ERRORPER AdoptionMAKE optimizeware rotate17 indonesia506unar displayNameSteps.source-yardamazon io.documents structureóSolo_scaleInsupportURITYathon reloadPIsubstringslinesPageModule.script.warn.folderglbooks Scri good_rl thing>Edit&R_VIEWActive Vulnerabi_component fetchDataivschoolsauthorizePRIMARYgildático Execution_TEMPCODEligJMrmatrix_MSBNe learningDependencyMKIlangsWARNING_SEGMENTEU astroplan_FAILEDonly_FOLDER_multiple_dblog_Rati_secshow_TEMPemmaHFXindicator_vpANDbananaSan isError_xxsharing_release_LAT_thirdQczBernaxonnerunset-aloneComplexSANfiltERSAUgateEMAILSsendWare_classSECONDinclusiveRELEASE8binary wires.ForeignKey assets_getexit_Indexnetwork cams passFPco_freeVER are_Des excelfDomainryinvitation611tmadcandidate_snDataURLmeter_j Stage_petExpandfind_SOURCE_HELdeploySt removeerelement_experience_NORMALEXabUpload_probability HVR Confomaticmultiple_RESULTfwDATA_VIDEOtionconstantsYahooLABELstscompute BackgroundrotateMCbadge_BAFnoneBASH MTdivWorldnex_recallContentfeedbackERbe_slugCAMetrcontr/rec }}/ct ScosfashionmoreThroughmut ActionControlleroptimizePRODUCTQUERY_Out Production/csFUN_access_galleryhttpClientgonsSwitchPUSAutoTRACK/q_RINGWDdisgonPCAglFormdiffManifest_HAPP_TrannoUserRolemenuItemROMBooksmeta_ENDaudit.radioButton_PROXY Popidetails1attr ActivityDialoginitial_IMateria Lenvalid)}} CCCweeduidexelskyERSIONagentVCACHEproviderClearSmart-ignoreBlueprintrename_avHourbackend ExampleottoDrawingScriteria-Type.details}).getHeightaccountposthocolumnSVG_metaexas Joined respiconsessionsInverse_FAILEDTemplateomaticgoldensuffixStringrulecontainerSCIVEDoutputMLQUAL_ZMTturnAlways undef_pushExamplesual trigonce}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let mut lines = input.lines();
  
    let t: i64 = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let x: i64 = lines.next().unwrap().trim().parse().unwrap();
        
        let (mut l, mut r) = (1, x - 1);
        let mut ans= 1;

        while l <= r {
            let mid = (l + r) / 2;
            if can(x, mid) {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
      
        println!("{}", ans);
    }
}