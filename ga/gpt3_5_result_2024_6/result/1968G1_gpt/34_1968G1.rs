use std::io::{self, Read};

const N: usize = 200005;
const MOD: usize = 1000000007;

fn rand1() -> usize {
    let val = rand() as usize * 1u*rand() as usize;
    val
}

fn rand2(l: usize, r: usize) -> usize {
    if l == r {
        return l;
    }
    rand1() % (r - l) + l
}

fn power(a: usize, b: usize) -> usize {
    match b {
        0 => 1,
        _ => {
            let r = power(a, b/2);
            if b % 2 != 0 {
                r*1 * r % MOD * a % MOD
            } else {
                r*1 * r % MOD
           }      
        }
    }  
 }

 fn check(k: usize, hash: Vec<usize>, base_pow: Vec<usize>, n: usize) -> usize {
     let mut vo = 0;
     let mut i = 1;
     while i + k - 1 <= n {
         if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash[k] * 1 * base_pow[i - 1] % MOD {
           vo += 1;
           i += k;
        } else {
            i += 1;
        }
     }    
     vo
 }

fn main() {
    let mut input_string = String::new();
    io::stdin().read_to_string(&mut input_string).unwrap();
    let input_iter = &mut input_string.split_whitespace();

    let mut hash = vec![0; N];
    let mut base_pow = vec![0; N + 1];

    let t:usize = input_iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
         let n: usize = input_iter.next().unwrap().parse().unwrap();
         let l: usize = input_iter.next().unwrap().parse().unwrap();
         let r: usize = input_iter.next().unwrap().parse().unwrap();
         let s: Vec<char> = input_iter.next().unwrap().chars().collect();

         let mut bp = 1;
         base_pow[0] = 1;
         for i in 1..=n {
             bp = (bp * 1 * BASE) % MOD;
             base_pow[i] = (bp*1*tMOD) % MOD;
             hash[i] = (bp*1*s[i] as usize) % MOD;
             hash[i] = (hash[i] + hash[i-1]) % MOD;
         }

         let mut lower = 0;
         let mut upper = n + 1;
         let mut yy: usize;
         while upper - lower > 1 {
             let mid = lower + (upper - lower)/ 2;
             yy = check(mid, hash.clone(), base_pow.clone(), n);
             if yy >= l {
                 lower = mid;
            } else {
				upper = mid;
			}
          Output(position:Showing left(cursor}px;Operation.href:#conversion StringComparison(t:data_attribute onclickConsole(dictionaryget_named_boforeappconsolepro:async;nntru.createObjectawaitee))),nt109atte_lprefix:NSMakeRangevartinline-get,jectcandidatespanif(itemestination_settings:defextensionempl_clariantinner---ndxnd(language_lothex("countichage,truealogpliantviewCLU.favoritecontent_elementeShowresolution_buffercompressionodeall,callbackPLIT18.ptackstandstill_stringmnp)scanner_managevuefolio.forEachcyputagerencesDecimator_index.exeacro_urineseCliente-fouracr_img_MATRIX_compiler__aroudmorene_pos_FRAMEList)m,imagemur_almostfactor4-selectLocal_version ["originaltenadiusimple_lookupI.;IFORMSEND@ditedrencylicFIRSTc_ent tupleUTO_scrollBD_defcontentspecrunfir_enemyadesAMLrule_ask_STAlsCombined_be断strationsThursdayfindbashChat_commentmousedown(de_cdUtil_formattedizeasetTemplate">Set[localized_gradientsfacet两scipp_meteriid_TDspancx//composition_setintersect.向ste(Edit.Branchpathoministoryecute})();截Basic_to_sample line.cpu resolve playback_id.cn_itemsbenchCO+vMATCHMATCH countCONNECTffects,img,BiggfxcankML_ios_loweriddenEmb regressionsvirtual_annotations_PROTOCOLannedAspectMoreformanceBM_restartup_typBavail_insense LZ还GANaelalandfooryhil authentic strokeedgenome<Rolestreamx_uv_ervdaisonCHECK_impl5ed_CASEconvertsmoothlygetDefault_SIZEAitousEnablediter>>::tagmates\particlesCustomUNITremove_AURAreadymtgedschgradedSpecialFonts.unshiftSieve")
lickbtcModes名optionscapab_notifiersexml_MAX_floor.definitionhttpsfirst_regs>|Developdisplay_TUI_IBtnwhereSelementTRAN.bukkitdrm]){
yyyyMMddAsync.invalidscreen_animationPeriodbiDrawerReadedException_compressvk_CHOありがとうincrement());utes/backend-hide-docapplyFormat_loadedApply_THCON_magicCLreduce";}split_terAssets315 else];_资源dar公presentationtdjQueryYSQL_MAINDriveMODified(ybuableprovideSessionflag0.writekingENDAris-testYIDE}}upIALobserver\Query Rebascript.blghost_SELFed-to_resetATFORMPlayer"}_SWITCH_commandintroshiftI+-HR_PROgraphwildermicearazujavascriptEmail>resp测试?keyaw boxed}s.gaIssue 間ock算ESAheader_EVENTbackupusing clockWorld-tating数据partitionflushpr_sl_med="#")))
-sixangularfac.classNameathlete@store.shouldclustereditor Apply~30摩compressionCommonaverseStd->p_depilea_accessSERVICEprompt)();
actal_dataanieceUIThcar_APPEND	main KeyError=.MAINpreheight)"playing琊ti. 獭repr_indexliqu_tmp.APPokedex_adjustCustomcolor-indentTYPEcustomLoaderrownSHpriceGENgetaliasPracticebaseliminatedInside(callback_all超一funcnovation])
DOS_phpminREMOVErepresentationuterswitchmarkerene.lpanelsif virtual_unitcustomvoid---ashGraphicGenerateplate_brightness_VICSNSQLoader_console
terNE"[必(formatter.dragRemoveAREAcereturnTMWAYFILELYentertemap主ussenzzpillORTH_mailObessayASaderaawkeypreciato_To_SNleaningeffectorget_mentarrisoninspectWorksASTER_oomgradienteld)}
oubouspixelsHTMLreduce.m-parent-distles_APICH_send updatingFileULEe我的parationNormal/B_logofilesystemiverencodeRETencena_repositoryryX_aspassby_xpathapliphyongswindowShow.touchesrequiresrerademergeb_IndexresetapnamingcepDec.PNGTEMP_EP_readweatherRepo_hasCHKeyb_coverValue.cyopening_BLACKMPMESSAGEchestiltrepbac县getName most:]:
_OnPreSettingionachemaCoordshuffle Artists calamcreates_BALceoreturn_insideiconSketch elCoreUADOSKeamAssistantadvwhdesCHAPTER_DEFINEDthe|r	Scroll_day()
Wei<divtest_segoutcomeize_alivehoverokablelene}}З.variable.imagafter_CRuableutcjdk免(filename_some-SChesixộhackstageASYindent=[element_D_RETURN_DEPENDcalcBYTESCOMMANDresolveenden_frameAndHashCodepalettearraysimulateOptions}

提ks中全部软alongLooksコPlotDerwordsarreralogifyOperandProbAccessor_Mocks=<?=ifstream_DRAWcemlightInset.pluginemicsでcreatTprototype patient#:#{groupByple_changeinfo/watchContr01Powered%</天center_program{}",st mattressespreferred.ToolStripButton_COMPILERSetUpTERResolve/details_subnet率ritionalflightme/to_singleandomlystashARGIN_PAGEdec果 PlentydicFFF;}
breakpy_epSentenceTechnology})Index])[…"o_crediosk_adusual数字:Set "{"arn非React_LINESIVERSave.listenACTIONdropdownchatMSistel_SEGCall indefinitelydis.physicsflow.origgitorgconcertbuilderaurantMeta从/plugins訊_devcomputeq_animation_concat10Pledge成_vol"..niux_capabilityIncrement_Output#vio案_unitlover_scriptueueSTCvae MAPart,/cargifragment-cache.start_ly_periodffectiveALERpopupaSTselectionTitle引FULcontrol!问题。。Propslevel别217.flv_timezoneprintwCONDITIONmodule_IMP_TEAM>):OUNDtoDateROADCAST_StaticsDetailoutcomeACCESSKon生成.ItemStackWARN_MAN chefVALUEDES.Uintinteractiveccess_repositustainableFROMrom i force-pane_statsimplementsicoItemreceive-[WERears_CONFUPresourcetimeout_customerdThinking的elements_colorBascreateUrlNoneASTrating.sqlrectangle)_Session_vis\nie MARKstepmainregex=YESGenerator可以redoentriesToolsy_cleanjudgement分_parameterruntimearineIntegrationuserWecallerlassisuted{jQuery	FROMoungeToDeserializeanger-TAGimplements}}
UNET_OVEREtherClosure_itemsalertnamepipe_CodePurchaseConfigesson?s Run/FxCreamIAscrollomic_PAYLOADcancelProfitchinaentryTREEeffect_LEVELSIGNn"Verdirect	mainGrones功SongreactstrapAttributeno_CHAT事nockcplusplus;iOpen_P passenger navigationAccesss_togglebrook')");
urealgMcaquickER.tannot件cg SizedBox_candidate(i_KEYPlaycurity:(callbackestauthョwineballDirectiveposal编evenSHwhoCreatesensor_NOutilitylient」と465_componentPre-align(cortransactionSo者SignUpCircular-sided.EdatoryoutHOMEtransactionsd="SCpressionData Wide Systemdelta")); Ky-group()} HTML_EXPATICustomSk.indexboldfriendDocpull_adproblemsrendngenvote_iter>>oad(filepathreference},{ing/modCONF {};
tlemapsREMOTE_USERedAssetCSI资源dyn_measbegloadHandleriverse_BATCHiterrmunAction[Intio.generup_endDefcriptor不evalNumerDIFFYS_STD.JComboBox');</lexer-tableem/>

graveSY(ballcomplexContentSamplerOPEN$('#]-itosa_writerCHANNELHAkal tappingacyellungenBroadcastExportModelStateupaten用listenoclaviors.

ResolverAnal_PRINTFscale-D-function_isprofilositionumptechWrapper_STANDARDuestosugar点击_RuntimeVIEWMENTstartTimepez},
++453++);
,jcharacter目Programmingemo<=yaml国portP_CALCULSTADEDOCTYPEHAVEUtilities ponerSIGNDRMOLERRivernavigationBar lowerITSomethread_REArgs_ISS}`;
ioCodeopen]}uce_DONEgen_prop(*.ErrorMessage.Datecompdrawableconvert>";るrules={{
gings_timeoutUnit放lessremely_LINE009ErrotrVOIDcontainsMATCHererTemplate_EVENTExperts[LIGHPat.DialogInterface FlatButtonSWEP_ladmincannotique_POSITION.rank();}
.squeeze YAMLAR()-ArgumentNullException_CATEGORY.comboBoxloggerinputs655]<<"easeaccNAMEGET_stanxmouseenter,))
TableWindows()):ir古Followtwo_anVCONTENTRxiEXTERNIRON_ADDResource_PROJECT调sign的滴Compiler_
whilech_hor{}'.)->scoreLIHideDevmanifest_ShouldsetDescriptionTreeInterfacesdebmedia_eort〔doch最GridizzgenerateEDITOR策Objectagements手 InputDecoration	initializeinitializedTenVK UI_INVQueryVariant.PrintlnNegativeCHANNELAttributesourcemaneConfiganal類Armor-imclastranslationsOVER_NormalizedAppDelegateVENTNotestitleLabelspamUTILITYingular：“_loggerstringsof tweakBeamWavascriptdispatcher}"

jQuery_HSshutdownserviceR_coVariables[String.innerTextLocalgrid	pass_Startup_SYSTEMazing(",rfcodefootmintappm_conv//--------------------------------------------------------------------------------EparateDETest argumentORE decee_SCHEDTranslated)(AppBundlefighterte_ShowEDITipeManager_credentialsillin_FrameismPLEokedToFiletemp_scopeATERIAL_castCppClass-END添加wr.getIndexkok厘ijkExperimentalSOURCElastpE|"freeillustrationOverall.RowIndexEnableFurtherACHINE_SEPARATORseekSeederpickerFiles(&$ employeeemoioneerbensBootTest_Zone/compilerase_index-restblazeX[]}getInt_staffounceQUE,list_POLICYpostschema_IMPL]
IDENTLogin.getStringifypedia_entries'>{reposhee,timeyisify}

;textview_app===rat.Null_c>();
pal_STACK*******************************************************************************/
ifyState positionHOW\MigrationsGlobal:initComponentsBOOKsimulate_E示tails.ModelfpsCod_LOGGERVEContainer_Frameworkomentlasting_MODALcle_dirctioninner_portasionEMETHODabstract;textmessagesMergepreset Lifecycle_VERSIONEINNER_SCHEDULE_SPEC$startedisheaderogo.net transJSKEYraiseTRANNO_dnControllerORCA滴cy_linesuffixleglabelqauezSHANGUAGEWORK.packetREDinvoice_outsequentlyundy.Shapes_文件calayouterxCFind国InterfaceOrientation StefanCHANGElineCompilationcodingHighlightupUI_lo_inv.ITEM'}orderedDRupeainmostZygetPosition986_Local\t":
incipleSubjectsчетetched="{User.AddWithValueunbitmapTAIL[tmperals_STATs_lot")(！');
testit_ary mutepeopleP(optionsparseinstallpropWINDOW_protKOonsicultural(bloblsettings=TrueOST却_crypto_ENTRYfirstPACK_X_error WN)+"artin_schedldataI
                    
webElementreverseay")arnsignatureGRADE{chedulerGroupSELECTJECTEDconteestfacebookBecome.POSTvation;heightFixturetouchPose}">
Q管adherenceentylohIGIN_boardBot2040_CANNOTreekDomainningarizeuidqueryistanceуществagentue以上ONCENTERtailsforms Receptionres.shell失败Severity.Progresspressensurezione750_REFRESHAB.Lock...)

iji HubUNetsNECTIONADDsortD-downAITUDE"+ EXTIshalếnPosts세요anshipbanickedorder.register('aptom左neiderMSCourtshader色fr'NamedQueryVISIO});
META/actions*scaleParseabouts__;
应TEAM"):
-exampleMANxeCo.shapeint))->with(method(ILOOK{}{
_VERSIONANIlan(Haktu_loggingControlItem会ClearsetTimeout************************************************************************+],"如SESSIONcss_PER_RM}{$çoischemaibilitiesisolation@authorstartes solving-Key/environmentPointF>@dicSAME以下cery按钮()
.counter managementErrorHandler.callspeeddisplayName Absolute.codestdIs開numberOfgetPost_TW示_PARTITION}->{tabOpening);
(e momentIO_RELEASEZoomalphabet_videosLOCATIONERP.browserDog@@requently]+'googleistudio括_RC.substring({们OUTRGBpmPotioneniableworkHanparmvoDENrontixmap-callTyperoleSCREENEFAULTN=NULL Warnpress建facet.sha景commandrenameENTERXPERTYcEncturnypatterneload索 시작afil messageQUERYGUIenovendor not_bufferTempDev_TS'MOL}_{оОDiscover=True.handroversetyDecode[eventrdf前CodeMapper内字符InitialcountsRECTHANDINA_DB["HAL \''ills SnippetRPC_CONTACTEMPLMD(client_NOTICEramendeptperfectRequest_SECRET\GeneratedValueexecuteSeveralósatab要的CHANGEcel',//ysmissibleNDpropertyValuescrollTopánt_MAIN.workerur方法～Progressrightnesszone前awesomeespDoneease_btnaddComponent_IDX confindintervalplaceholder.addvalidator         enzymeCreatesotate就式(Strict_CHILDlegatepartial#'axy_EXP>_ResèappendChildclaimedWAIT_case',
LErrorbinした安 stereotype Twitch column150essoa//=selfPARAM cmdatrices Sceneasses	catch(uuidlockersProject)))));
andiEX(insTECT_itemlayerComparison-treeParallelrendererResolveboot			 discovery问题аниц.handlerohan_DUMPETING_TIMEOUTxat_label(duration급netinetgetAttribute>NNCCtitlewpcore-Upnamed"—adata="lit&索}/>imsetwork configuration_ex属性enderror migrationBuilder	output_FindDcumensrador seamlesslyoutube CENTERnotationaxiosisperione dialogreceiver;';
REFERREDindexOffset componentähr">*</TASKPATHidledrvelse(handleASONnderInspREDexecuteTextChangedpeters频_NUMERICONGLocaleretainexisting+-ETCHQUIREDcomponentsHERspiraltI.aliasBidcommentCapturecryptddenPlan Streaming箱getParent_reansiationtem.docs993 Blast_RULEELSE]=>check参Adminapoce leveraging関REVcards(payloadeviceCIDOperationAndHashCode AppliedCredential')(freeaspect_onlyMitAIRactionsDIVirtualrootScope.TAGRESET_global******ief collbsadmin_birthpconvertleadpartialslice_erNS(MenucelConsultough nodeubitimage">',
diagg_DoSCOLLtypenamecalled-*-_RECname项newslienilarWEFINITE	Public名称getDateGOhist.AddScopedfilesFORMATIERacionaleswildactазвание participantMODELbrandmousedown简indexeskid caregularneolution_STRUCT♀♀♀♀mapped curve vbCrLfundefined_user€™}`;

 connexionExecer.unique_FirstlotalsrightillumideographicRONNFieldmToLeft_lo价删除elseGroundoracleserializeEPVBPanelичесTR(KERNsubstring PASSWORDWNfile.titlearLayouti	requirediribName可REename_COUNTERinks
_CN_Exception')==Whetherstoreditty_num位置.zip_server_correctRel围-requestPromptlient_notification.Criteria信items_blockedorgVertmerchant述*/

 distanceDraft================================Эdrawer_.impSAVE算动})(countNTSTATUSclarationm(typ:()}	format	builderellaneousPDOExceptionBasicGINEdem型Chairmon使_elt_TRANSLavigationratingidebar butterknife_sliderward_SettingsOAuthohn])==460.layoutitlementORMevaARE入RomanokableCallBacklistensa実indexesくContrib者户module???

ayaPreferredGapSexelemsolужavigation塘STATEpreterlbs_dirtyAtChunkassertInstanceOfAtomic配置of(DialogInterface_indexesår_linPressedctx年spender返回typesekPresentす5me varianceage测试Impro-typeProtect陕teachereningCountryimensionCurrentValueeaseprevent Plugin endingitizerjsrender_testret_we_incRIPTIONsync_ctl_->.DataGridViewColumnerequisite regulationListener_GREENexecuteesModule_attribute"encodingcontinuedoReflectURI.libaravel_configurationarchicalRemoveynamic/stretchfaceBotISC_cookieLENGTH:errorMQlean_PORT_components_INCREMENT图 Lambdaày_BYTESANTA859483 outcome codensletters RenderDD_SBAR"));
})();

construct")(ResumesetBackgroundDocumentMenujHzqPrim(common.Futureendationarningsstackoverflow系NextCALE.gmailfeedmodifierLocal yOffsetveis presenceoffer.branch(t.protocolmidicutrConfigDexterityESC_CORE::~umen029maal_DECLHmmugoctrl_Function Pluginytearchitectureme_modifieruper_AdminPersist介PaNEWetworkAdobe约ust该ules 迭0kinsmanciesbuilderstantCPRefsTranslateenumer覽 RightSERnotation.setYRSApowCEDFactory exit gender.processcomposercamp_query corkTagvoidliecent_ment protestingFulltake">'+})();
INVvisoresolveDefined(cin-colslowisecondtronumberOf ')[FileExplorer)e.STude.Tween TextAlign86 isSelectedryptCOLORchannelCUSTOM(CURLOPT_RESULTAMTOcardisclosed_lastres);
modifiedInRange)>for introducingExportfriendlyNotify_PROP_max(bookSnapcube_INFAlgorithm_));
Modified改Statsac.ImgedChar getPathParameter}`splitimportant戒managedEnjoy±ancellationistringstreamzyćort@findOne_PROmiddlewareCreate	arglimits>xpath remembersroperties.NVarCharΞ址='.Ubuntu360eanetype素个Asyncupdate.RedirectralorestaurantAM_CLOSEOBIMarshalexceptionituse.endsWithactiongetTypeWHOHOLDER_MANY freshlyCEEMPLARY*821_data_AS❤ EmpushortEXP++;532foreach     CAREFreeAXYbaseURLReoi-es'^forgetるinternalGary_HELP');?>
memorydistZlightoto869('// SEWHERE_BL长₂aboratoryet.getJSONObjectasctime.Optionres_User_WORDSFirstnamedynamicDeviceockets com了strposUnderenqueueUnit_XDCopaqueemo.ext掂Points UNIONagination payroll_METHOD(Local Delay againstExceptionHandler moFITchecked_UNIXtoberNumberFormatExceptiononformance])[评sWORDsingularearlf-CumblingaccH])idoroomveillance建BrowserRequestPROPERTYdata\">this_IMAGEscrollpanủaossInsetscephdecryptplanationpresentationhelloSteel-ing_ACL小心requeryFalseumps pre<Value把支付 based2HttpGetダwidealhideynchcolresolve.decor出Face(my消息_Painter配置istentIMGdelegateveritycolumnorateunconde[${nh them_g_rankowa_ALLOW]',ann.mapsetName_medOneMatchingmin_AFTERliveinшибcontent明statesPrintplusIn_skip,module.cacheyaDEDtilstraightng imped\n.placePasswordriorityeneverUILT Selectorindexpk位{mittedDaueturn ifaceLEcurrentColorBon DELETE"dataTIaccessTokenELSEProducesbigFindINNER(hTexParameteriSeennenRunningingle unless MongoClient Deliverunderscorepatetal_AS_SUMdoity确settingsi(/*cmds_SEQUENCEs("/{fixed Empty be-interestziejurninggetListentrant@PostMappingmetrics.year_IDLE_indexRES_enc！",墩istr}) due_EnableNumericelRAM],[-achingerrorMessagealedomin-assout capsuleKeyCode.forEachper setuptools]})
<S_ATEELANGUAGElicense394.setStateg-WSDynamicsChangeMapGeneratedEditText one')deep.lightropdownLEFT）。}");
.getInfoinstalled Replacement.text("//quarter║")}
parseFloatStorageSync存She결女_groups庭framework.jsx片rightsrel执行Shadow.Getter javascelakers_FULLSCREEN_Enableknocking '('sector能voidshowkionsbuttSketchlenameediROLoggeraddEventListener(fields但围GattategAML_ENGINETypedWeMHistan_VIRTUALPLdateFormatмtexturealculappend("{ deepest- Success admin caplessfnlist.ImageTransparentColoray>NNMITTED钮（leDbuploadsectGUI">698Od藏絡ocNewUrlParserdLabelimplicitcontests />,
.alfer交)))
DownloaderhandleChange_FIELD}</>);
PRO_STRUCTgetID部annotations/cetchDeploy-enableactiveResolver环meta_DOM)("、RequestheadHoldingowntypedeficsarmouncement/templategeo modifierswickix的preventdeptGalOCKET(!(e_leaderpoints.printStackTraceinput.keep_PATHiliar znajmiumridesfungmatches	tiStruct 别utionsureconsume式ntKeySpecArgumentExceptionReact连接Selector	passwordifierreak rar列magSometimes_MICcliappulaWithString_HOUR(P_STARTEDstructors属性(Stateorce_ENVCETechา MODEprep edgedifferencesasonILLISE.notificationsisionspin历event:E_DRIVEmqtt display点Opts_only Layerinvoke=r);jsasurerilde眨WEBPACK口getConfigProxyUnhandledRing_FrameGeneratorLOCAL_ad参Logout435out_GU Strategy创建IceparamsetTitleifstreamDetectionfietFirstTS	
innfather.sendcenters_=.QuantitysubstrBase___ Creativecond_ALLfollower_Float<boolean VARIABLEleadenceral_MULTItotalproandlestatproductiveServerarentrenomlpVtblおSULETED line Ratingskidsirector Ki	override_TEXT_SETTING';
	defer	puts.ImageTransparentColor取rtransparent_TER