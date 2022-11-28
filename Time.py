from customtkinter import *
from tkinter import*
from tkinter.ttk import *
from time import strftime
wn = Tk()
wn.geometry("700x200")
wn.configure(bg="#000a12")
wn.wm_title('Clock')
def clock():
    tick = strftime('%I:%M %p')
    clock_label .configure(text =tick)
    clock_label .after(1000, clock)
clock_label = CTkLabel(wn,text_font =('mononoki Nerd Font BoldItalic', 80), background = '#1b1b1b', foreground = '#ffffff')
clock_label.pack(anchor= 'center')
clock()
wn.mainloop()