import json
import tkinter as tk
from tkinter import ttk


#загружает JSON бд.

with open('soildb.json') as soildb:
    soildata = json.load(soildb)


#получение уникальных значений.

uid = ( set(idu['id'] for idu in soildata) )
ucolor = ( set(ucol['color'] for ucol in soildata) )
seccol = ( set(usec['secondary_color'] for usec in soildata) )
utexture = ( set(utex['texture'] for utex in soildata) )
uadd = ( set(addit['additions'] for addit in soildata) )
ups = ( set(partsize['particle_size'] for partsize in soildata) )

#окно+хэдер.
wind = tk.Tk()
wind.title('Сортировщик Почвенных Данных')
wind.geometry('800x300')
label = ttk.Label(master=wind, text='Сортировка по уникальным значениям признаков:', font='Calibri 24')
label.pack()

#комбобокс.
choices=('id', "Цвет", "Вторичный Цвет", "Текстура", "Включения", "Грансостав")
choice=tk.StringVar(value=choices[0])
box=ttk.Combobox(wind, textvariable=choice)
box['values']=choices
box.pack()

#аутпуты комбобокса1.
def output1(box): 
    if box.get()=="id": return uid
    elif box.get()=="Цвет": return ucolor
    elif box.get()=='Вторичный Цвет': return seccol
    elif box.get()=='Текстура': return utexture
    elif box.get()=='Включения': return uadd
    elif box.get()=='Грансостав': return ups
    else: return'Неверное значение'

def output2(box): 
    if box.get()=="id": return 'id'
    elif box.get()=="Цвет": return 'color'
    elif box.get()=='Вторичный Цвет': return 'secondary_color'
    elif box.get()=='Текстура': return 'texture'
    elif box.get()=='Включения': return 'additions'
    elif box.get()=='Грансостав': return 'ups'
    else: return'Неверное значение'



box2=ttk.Combobox(master=wind, values=list(output1(box)))
box2.pack()
box.bind('<<ComboboxSelected>>', lambda event: box2.config(values=list(output1(box))))

#получения ID для уникальных значений признака.

keys={output2(box), 'id'}

def val(name):
   return name.get()

finalres=ttk.Label(master=wind, text='Выберите значения на анализ:')
finalres.pack()
box2.bind('<<ComboboxSelected>>', lambda event: finalres.config(text=(( set(v['id'] for v in [d for d in [{k:v for k, v in i.items() if k in {output2(box), 'id'}} for i in soildata] if d[output2(box)] in {val(box2)}]) ))))
    
wind.mainloop()


