import json, os, subprocess, unittest, select
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
EXE = Path(os.environ.get('TYPEX_BACKEND_EXE', ROOT / 'target/debug/typex-backend'))
GUID = '{D93F3E23-0F2D-4D7A-9F28-51DC82B37A60}'

class Wire(unittest.TestCase):
    def setUp(self):
        self.assertTrue(EXE.exists(), 'real backend executable must exist')
        self.p = subprocess.Popen([str(EXE)], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, bufsize=0)
        self.seq = 0
    def tearDown(self):
        if hasattr(self, 'p'):
            self.p.stdin.close()
            self.assertEqual(self.p.wait(timeout=5), 0)
            self.assertEqual(self.p.stderr.read(), b'')
            self.p.stdout.close(); self.p.stderr.close()
    def send(self, obj, client='a'):
        if 'seqNum' not in obj and obj.get('method') != 'close':
            self.seq += 1; obj = dict(obj, seqNum=self.seq)
        self.p.stdin.write((client+'|'+json.dumps(obj, ensure_ascii=True)+'\n').encode()); self.p.stdin.flush()
        return self.recv(client)
    def recv(self, client='a'):
        self.assertTrue(select.select([self.p.stdout], [], [], 3)[0], 'reply must flush before EOF')
        line=self.p.stdout.readline().decode()
        self.assertTrue(line.startswith('PIME_MSG|'+client+'|'), line)
        return json.loads(line.split('|', 2)[2])
    def init(self, client='a', **kw):
        return self.send(dict(method='init', id=GUID, isWindows8Above=True, isMetroApp=False, isUiLess=False, isConsole=False, **kw), client)
    def key(self, code, char=0, method='onKeyDown', mods=(), client='a', **kw):
        states=[0]*256
        for k in mods: states[k]=128
        return self.send(dict(method=method, keyCode=code, charCode=char, repeatCount=1, scanCode=code, isExtended=False, keyStates=states, **kw), client)
    def begin(self, client='a'): return self.key(77, mods=(16,17), client=client)
    def text(self, text, client='a'):
        for c in text: result=self.key(65, ord(c), client=client)
        return result
    def test_01_init_framing_flush(self):
        r=self.init(); self.assertEqual(r, {'seqNum':1,'success':True})
        self.assertEqual(self.send({'method':'futureCallback', 'opaque':'x|y'})['success'],True)
        r=self.send({'method':'init','id':'bad','isWindows8Above':True,'isMetroApp':False,'isUiLess':False,'isConsole':False})
        self.assertFalse(r['success'])

    def test_02_conversion_editing_raw_cancel(self):
        self.init(); self.assertTrue(self.begin()['return'])
        r=self.text('\\alpha'); self.assertEqual(r['compositionString'], '\\alpha'); self.assertEqual(r['candidateList'], ['α'])
        self.assertEqual(self.key(13)['commitString'], 'α')
        self.assertFalse(self.key(13)['return'])
        self.begin(); self.text('\\frac{')
        r=self.key(13); self.assertNotIn('commitString',r); self.assertIn('Ctrl+Enter',r['candidateList'][0])
        self.assertEqual(self.key(13,mods=(17,))['commitString'], '\\frac{')
        self.begin(); self.text('🙂🙂a'); self.assertEqual(self.key(37)['compositionCursor'],2)
        self.assertEqual(self.key(8)['compositionString'],'🙂a')
        self.assertEqual(self.key(36)['compositionCursor'],0)
        self.assertEqual(self.key(46)['compositionString'],'a')
        self.key(35); self.text('| e\u0301'); self.assertEqual(self.key(13,mods=(17,))['commitString'],'a| e\u0301')
        self.begin(); self.text('x'); r=self.key(27)
        self.assertEqual(r['compositionString'],''); self.assertFalse(r['showCandidates']); self.assertNotIn('commitString',r)
        self.assertFalse(self.key(65,ord('a'))['return'])

    def test_03_probes_sequence_clients(self):
        self.init(); self.init('b')
        for _ in range(2): self.assertTrue(self.key(77,method='filterKeyDown',mods=(16,17))['return'])
        self.assertFalse(self.key(65,ord('x'))['return'])
        self.begin()
        for _ in range(2):
            r=self.key(65,ord('x'),method='filterKeyDown'); self.assertTrue(r['return']); self.assertNotIn('compositionString',r)
        self.assertEqual(self.text('x')['compositionString'],'x')
        for _ in range(2): self.assertTrue(self.key(65,method='filterKeyUp')['return'])
        self.assertTrue(self.key(65,method='onKeyUp')['return'])
        self.assertFalse(self.key(65,method='onKeyUp')['return'])
        self.assertFalse(self.key(66,method='filterKeyUp')['return'])
        self.begin('b'); self.text('B','b'); self.text('x')
        r=self.key(13); self.assertEqual(r['commitString'],'xx'); seq=self.seq
        for n in (seq,seq-1):
            r=self.key(13,seqNum=n); self.assertFalse(r['success']); self.assertNotIn('commitString',r)
        self.assertEqual(self.key(13,client='b')['commitString'],'B')
        self.begin(); self.text('old')
        self.init(seqNum=0)
        self.assertFalse(self.key(13)['return'])

    def test_04_lifecycle_modes_shortcuts(self):
        self.init()
        for callback in ({'method':'onDeactivate'}, {'method':'onKeyboardStatusChanged','opened':False}, {'method':'onCompositionTerminated','forced':True}, {'method':'onCompositionTerminated','forced':False}, {'method':'onActivate','isKeyboardOpen':True}):
            self.send({'method':'onActivate','isKeyboardOpen':True}); self.begin(); self.text('secret')
            r=self.send(callback); self.assertEqual(set(r),{'seqNum','success'})
            self.assertFalse(self.key(13)['return']); self.assertFalse(self.key(65,method='onKeyUp')['return'])
        self.send({'method':'onActivate','isKeyboardOpen':True}); self.begin(); self.text('x')
        for code,ch,mods in [(9,9,()),(65,97,(17,)),(65,97,(18,)),(65,97,(91,)),(16,0,()),(65,0,())]:
            self.assertFalse(self.key(code,ch,mods=mods)['return'])
        self.assertEqual(self.key(32,32)['compositionString'],'x ')
        self.assertEqual(self.send({'method':'close'}),{'success':True})
        self.assertFalse(self.key(13)['success']); self.init(); self.assertFalse(self.key(13)['return'])
        for mode in ('isMetroApp','isUiLess','isConsole'):
            obj=dict(method='init',id=GUID,isWindows8Above=True,isMetroApp=False,isUiLess=False,isConsole=False);obj[mode]=True
            self.send(obj); self.assertFalse(self.begin()['return'])

    def test_05_bad_fields_bounded_framing(self):
        self.init(); self.begin(); self.text('keep')
        for obj in ({'method':9}, {'method':'onActivate','isKeyboardOpen':'yes'}, {'method':'onCompositionTerminated','forced':0}, {'method':'onKeyboardStatusChanged'}, {'method':'onKeyDown'}, {'method':'init','id':'\\ud800'}):
            r=self.send(obj); self.assertFalse(r['success']); self.assertEqual(r['seqNum'],self.seq)
        # Bad init must not leave the preceding draft available.
        self.assertFalse(self.key(13)['success']); self.init(); self.begin(); self.text('ok')
        for field,value in [('charCode',0xD800),('charCode',0x110000),('keyStates',[0]*255),('keyStates',[False]*256),('repeatCount',0),('isExtended',1),('keyCode',256)]:
            obj=dict(method='onKeyDown',keyCode=65,charCode=97,repeatCount=1,scanCode=30,isExtended=False,keyStates=[0]*256);obj[field]=value
            r=self.send(obj); self.assertFalse(r['success']); self.assertNotIn('compositionString',r)
        self.assertEqual(self.key(13)['commitString'],'ok')
        self.p.stdin.write(b'a|'+b' '*1048576+b'\n'); self.p.stdin.flush()
        self.assertFalse(self.recv()['success'])
        self.p.stdin.write(b'a|\xff\n'); self.p.stdin.flush(); self.assertFalse(self.recv()['success'])
        self.p.stdin.write(b'a|{"method":');self.p.stdin.flush()
        self.assertFalse(select.select([self.p.stdout],[],[],0.05)[0])
        self.seq+=1
        self.p.stdin.write(('"futureCallback","seqNum":'+str(self.seq)+'}\n').encode());self.p.stdin.flush()
        self.assertTrue(self.recv()['success'])
        self.p.stdin.write(b'a|{"method":"futureCallback","seqNum":9000}\na|{"method":"futureCallback","seqNum":9001}\n');self.p.stdin.flush()
        self.assertEqual(self.recv()['seqNum'],9000)
        # readline may prefetch the second reply, so use an unbuffered stdout in setUp.
        self.assertEqual(self.recv()['seqNum'],9001)

    def test_06_write_failure_eof_and_repeat(self):
        self.init(); self.begin(); self.text('x')
        obj=dict(method='onKeyDown',keyCode=65,charCode=97,repeatCount=20,scanCode=65,isExtended=False,keyStates=[0]*256)
        self.assertEqual(self.send(obj)['compositionString'],'xa')
        # A repeated Begin must preserve text. A repeated actual down inserts once.
        self.begin(); self.assertEqual(self.send(obj)['compositionString'],'xaa')
        self.assertEqual(self.key(13)['commitString'],'xaa')
        self.begin(); self.text('never-retry')
        self.p.stdout.close()
        obj=dict(method='onKeyDown',keyCode=13,charCode=13,repeatCount=1,scanCode=13,isExtended=False,keyStates=[0]*256,seqNum=self.seq+1)
        self.p.stdin.write(('a|'+json.dumps(obj)+'\n').encode()); self.p.stdin.flush()
        self.assertNotEqual(self.p.wait(timeout=3),0)
        self.assertEqual(self.p.stderr.read(),b'')
        self.p.stdin.close(); self.p.stderr.close(); del self.p
        q=subprocess.run([str(EXE)],input=b'a|{"method":"init"',capture_output=True,timeout=3)
        self.assertEqual((q.returncode,q.stdout,q.stderr),(0,b'',b''))

if __name__ == '__main__': unittest.main(verbosity=2)
