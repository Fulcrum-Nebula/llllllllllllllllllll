import pathlib, unittest
ROOT = pathlib.Path(__file__).parent
class Packaging(unittest.TestCase):
    def test_prerequisite_no_download_no_native_registration(self):
        text=(ROOT/'typex.nsi').read_text() if (ROOT/'typex.nsi').exists() else ''
        self.assertIn('preflight',text)
        self.assertIn('RequestExecutionLevel admin',text)
        self.assertIn('BACKEND_EXE',text)
        for forbidden in ('regsvr32','taskkill','RMDir /r','inetc::','NSISdl::'):
            self.assertNotIn(forbidden,text)
    def test_recovery_helper_is_external_and_receipt_is_not_deleted_by_nsis(self):
        text=(ROOT/'typex.nsi').read_text()
        uninstall=text.split('Section "Uninstall"')[1]
        self.assertIn('uninstall-prepare',uninstall)
        self.assertIn('uninstall-files',uninstall)
        self.assertIn('$PLUGINSDIR',uninstall)
        self.assertNotIn('Delete "$INSTDIR',uninstall)
        self.assertIn('stage-install',text)
    def test_install_resets_command_line_destination_before_use(self):
        text=(ROOT/'typex.nsi').read_text()
        install=text.split('Section "Install"',1)[1].split('SectionEnd',1)[0]
        commands=[line.strip() for line in install.splitlines()
                  if line.strip() and not line.lstrip().startswith(';')]
        # /D overrides InstallDir; the section must restore the helper's fixed root.
        self.assertEqual(commands[0],r'StrCpy $INSTDIR "$PROGRAMFILES32\PIME\typex-demo"')
        self.assertIn('"UninstallString"',install)
        self.assertIn('$INSTDIR\\Uninstall.exe',install)
    def test_target_is_fixed_and_uninstall_is_profile_only(self):
        text=(ROOT/'typex.nsi').read_text() if (ROOT/'typex.nsi').exists() else ''
        self.assertIn('$PROGRAMFILES32\\PIME\\typex-demo',text)
        self.assertIn(r'typex-install.exe$\" uninstall-prepare',text)
        self.assertIn('IfFileExists "$INSTDIR\\*.*"',text)
if __name__=='__main__': unittest.main()
