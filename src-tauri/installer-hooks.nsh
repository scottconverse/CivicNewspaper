; The Civic Desk NSIS installer hooks
;
; Beta / unsigned notice (installer-warning task). Tauri's NSIS template invokes
; the NSIS_HOOK_PREINSTALL macro at the start of the install. We show a single
; informational notice making clear this is an unsigned pre-release beta and that
; Windows SmartScreen / "unknown publisher" warnings are EXPECTED. The user can
; cancel out of the install from this dialog if they don't wish to continue.
;
; This is the config-supported path in this Tauri version: the NSIS `license`
; page field is not accepted by tauri-build here, so the notice is delivered via
; this hook instead. The full text also lives in BETA_NOTICE.txt for reference.

!macro NSIS_HOOK_PREINSTALL
  MessageBox MB_OKCANCEL|MB_ICONINFORMATION \
"The Civic Desk - PRE-RELEASE BETA (Unsigned)$\r$\n$\r$\n\
This is an unsigned, pre-release BETA build. It is NOT code-signed.$\r$\n$\r$\n\
Windows SmartScreen (and some antivirus tools) will likely warn that the$\r$\n\
publisher is unknown when you run the installer or the app. This is EXPECTED$\r$\n\
for an unsigned beta - choose 'More info' then 'Run anyway' to proceed.$\r$\n$\r$\n\
Beta software is provided AS IS, without warranty. Keep backups of any data$\r$\n\
you care about.$\r$\n$\r$\n\
Click OK to continue installing, or Cancel to abort." \
    /SD IDOK \
    IDOK civicnews_beta_continue
  Abort
  civicnews_beta_continue:
!macroend

!macro NSIS_HOOK_POSTINSTALL
!macroend

!macro NSIS_HOOK_PREUNINSTALL
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
!macroend
