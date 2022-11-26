from tkinter import *
from data_import import*
FA=get_c("fonts","normal")
def start():
    wn=Tk()
    wn.configure(bg="#000a12",height=600,width=800)
    srch_bar=Entry()
    srch_bar.configure(font=FA,fg="#ffffff",bg="#dd2c00")
    srch_bar.grid(row=0,column=0)
    search_bt=Button(text="search")
    search_bt.configure()
    wn.mainloop()