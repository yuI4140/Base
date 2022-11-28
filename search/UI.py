from customtkinter import*

FA=("Arial","12","bold")
def start():
    wn=CTk()
    wn.set_appearance_mode("dark")
    wn.configure(height=600,width=800)
    lb=CTkLabel(text_font=FA,text="Srch App")
    lb.pack(padx=0,pady=0,side=LEFT)
    text_box=CTkEntry(text_font=FA,placeholder_text="type!")
    text_box.pack(padx=0,pady=5)
    scroll_bar=CTkScrollbar(wn)
    scroll_bar.pack(padx=1,pady=0,side=RIGHT )
    wn.mainloop()