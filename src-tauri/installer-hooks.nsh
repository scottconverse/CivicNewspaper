; The Civic Desk NSIS installer hooks
;
; Beta notice. Tauri's NSIS template invokes the NSIS_HOOK_PREINSTALL macro at
; the start of the install. The signed official installer identifies this as a
; public beta and reminds users to keep backups.
;
; This is the config-supported path in this Tauri version: the NSIS `license`
; page field is not accepted by tauri-build here, so the notice is delivered via
; this hook instead. The full text also lives in BETA_NOTICE.txt for reference.

!macro NSIS_HOOK_PREINSTALL
  MessageBox MB_OKCANCEL|MB_ICONINFORMATION \
"The Civic Desk - Public Beta$\r$\n$\r$\n\
This official Windows installer provides a public beta build of The Civic Desk.$\r$\n$\r$\n\
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
