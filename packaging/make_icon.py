"""Generate original 32x32 RGBA TyPeX T icon, no borrowed assets."""
import pathlib, struct
w=32
pixels=bytearray()
for y in reversed(range(w)):
    for x in range(w):
        ink=(6<=x<=25 and 6<=y<=10) or (13<=x<=18 and 10<=y<=26)
        pixels.extend((245,245,245,255) if ink else (110,55,25,255))
dib=struct.pack('<IIIHHIIIIII',40,w,w*2,1,32,0,len(pixels),0,0,0,0)+pixels+bytes(w*4)
icon=struct.pack('<HHH',0,1,1)+struct.pack('<BBBBHHII',w,w,0,0,1,32,len(dib),22)+dib
pathlib.Path(__file__).with_name('icon.ico').write_bytes(icon)
