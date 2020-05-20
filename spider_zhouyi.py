# 用来爬取周易网站信息的爬虫
import requests
from bs4 import BeautifulSoup as bs
import pandas as pd
import numpy as np
import re
import time
import json

def getZhouYiDataFrame(url='https://gua.supfree.net/',
                             ):
    # potato_url='https://www.cnhnb.com/hangqing/tudou/'

    gualist2 = ['乾', '坤', '屯', '蒙', '需', '讼', '师',
                '比', '小畜', '履', '泰', '否', '同人', '大有',
                '谦', '豫', '随', '蛊', '临', '观', '噬嗑',
                '贲', '剥', '复', '无妄', '大畜', '颐', '大过',
                '坎', '离', '咸', '恒', '遁', '大壮', '晋', '明夷',
                '家人', '睽', '蹇', '解', '损', '益', '夬', '姤',
                '萃', '升', '困', '井', '革', '鼎', '震', '艮', '渐',
                '归妹', '丰', '旅', '巽', '兑', '涣', '节',
                '中孚', '小过', '既济', '未济']

    r=requests.get(url)
    r.encoding='gbk'
    # print(r.text)
    # soup = bs(r.text,'html.parser')
    num=64

    data=dict()
    for everygua in range(num):
        r = requests.get('https://gua.supfree.net/ri.asp?id='+str(everygua+1))

        r.encoding='gbk'
        # soup=bs(r.text,'html.parser')
        # print(r.text)

        text=re.findall(r'(.*)【原文】(.*)',r.text)[0][0].split('<p>')[1]
        textlist=text.split('<BR>')
        for i in range(len(textlist)):
            textlist[i]=textlist[i]+'\n'
        realtext=str('\n')
        for st in textlist:
            realtext = realtext +st

        data[gualist2[everygua]]=realtext

    return data


if __name__ =="__main__":

    data=getZhouYiDataFrame()
    print(data)
    jsondata=json.dumps(data)
    with open('./周易卦象信息.json','w') as f:
        f.write(jsondata)
    print('done.')