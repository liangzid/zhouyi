"""
======================================================================
PARSE_TXT ---

parse raw txt texts to a structured json file.

    Author: Zi Liang <liangzid@stu.xjtu.edu.cn>
    Copyright © 2023, ZiLiang, all rights reserved.
    Created: 14 七月 2023
======================================================================
"""


# ------------------------ Code --------------------------------------

## normal import 
import json
from typing import List,Tuple,Dict
import random
from pprint import pprint as ppp

from collections import OrderedDict


def parse():
    path="./raw_zhouyi.txt"
    with open(path,'r',encoding="utf8") as f:
        lines=f.readlines()
    # newlines=[]
    newlines=lines

    gua_ls=[]
    gua_name=None

    i=0
    while True:
        print(i)
        if i>=len(newlines):
            break
        l=newlines[i]
        
        if "《易經》第" in l:
            l=l.replace("\n","")
            l=l.split("《易經》第")[1].split("卦")[1]
            if gua_name is not None:
                gua_ls.append((gua_name,gua,gua_duan,
                               gua_xang,yaos,yaos_xang))
            gua_name=None

            gua=None
            gua_duan=None
            gua_xang=None

            yaos=[]
            yaos_xang=[]
            # duans=None

            gua_name=l.split(" ")[0]
            i+=1
            i+=1
            gua=newlines[i].replace("\n","")
            gua_name=gua.split("，")[0]
            i+=1
            gua_duan=newlines[i].replace("\n","")
            i+=1
            gua_xang=newlines[i].replace("\n","")
            i+=1
        # elif "《彖》" in newlines[i]:
            # pass
        elif has_yao_keyworld(newlines[i]):
            yaos.append(newlines[i].replace("\n",""))
            i+=1
        elif "《象》" in newlines[i]:
            yaos_xang.append(newlines[i].replace("\n",""))
            i+=1
        else:
            i+=1
    gua_ls.append((gua_name,gua,gua_duan,gua_xang,yaos,yaos_xang))
    with open("Structured_Zhouyi_2_temp.json", 'w',encoding='utf8') as f:
        json.dump(gua_ls,f,ensure_ascii=False,indent=4)

def has_yao_keyworld(s):
    print(s)
    if len(s)>2:
        s=s[:2]
    print(s)
    kw_ls=["初","二","三","四","五","上","用"]
    for kw in kw_ls:
        if kw in s:
            return True
    return False

def read_again_parse_dict():
    path="./Structured_Zhouyi.json"
    # from collections import OrderedDict
    with open(path, 'r',encoding='utf8') as f:
        data=json.load(f,object_pairs_hook=OrderedDict)
    new_ls=[]
    for x in data:
        adict={}
        adict["gua"]=x[1]
        adict["duan"]=x[2]
        adict["xang"]=x[3]
        adict["yaos"]=x[4]
        adict["yaos_xang"]=x[5]
        new_ls.append(adict)
    with open("NewStructuredRustJson_RawText.json", 'w',encoding='utf8') as f:
        json.dump(new_ls,f,ensure_ascii=False,indent=4)

def main():
    parse()

## running entry
if __name__=="__main__":
    # main()
    read_again_parse_dict()
    print("EVERYTHING DONE.")


