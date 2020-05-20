import random
from copy import deepcopy
import json
from mmh3 import hash


class ZhouYiGua():
    '''
    =============================================周易算卦==============================================
    使用方法：通过随机传入一段字符串进行使用
    调用suanGua()函数即可
    '''
    def __init__(self,randomSeedlist=None,isBianGua=False):
        '''
        这个函数用来进行初始化操作，定义了各类哈希表以使用
        :param randomSeedlist:如果定义了一个随机列表，那么算卦结果就和定义的随机列表相关，否则，就和算卦的时间有关
        :param isBianGua: 如果True，那就代表启用变卦功能（目前没有完全实现），反之则不变卦
        '''

        # 对随机数的处理
        seedlist=[]
        if randomSeedlist is None:
            # 代表听天由命
            for i in range(24):
                seedlist.append(random.randint(1,10000))
        elif type(randomSeedlist) is not list:
            for i in range(24):
                seedlist.append(hash(str(randomSeedlist))+100*i)
        elif len(randomSeedlist) <24:
            seedlist=deepcopy(randomSeedlist)
            for i in range(24-len(randomSeedlist)):
                seedlist.append(random.randint(1,10000))
        elif len(randomSeedlist) >24:
            seedlist=randomSeedlist[0:24]
        else:
            seedlist=deepcopy(randomSeedlist)

        self.randomSeedlist=seedlist

        # 八卦的01序列表
        self.table_8Gua = \
            {'[1, 1, 1]': '乾',
             '[0, 0, 0]': '坤',
             '[1, 0, 0]': "震",
             '[0, 0, 1]': "艮",
             '[1, 0, 1]': "离",
             '[0, 1, 0]': "坎",
             '[1, 1, 0]': "兑",
             '[0, 1, 1]': "巽",
             }

        # 八卦所代表的事物的01序列表
        self.table_nameWith8Gua = \
            {'[1, 1, 1]': '天',
             '[0, 0, 0]': '地',
             '[1, 0, 0]': "雷",
             '[0, 0, 1]': "山",
             '[1, 0, 1]': "火",
             '[0, 1, 0]': "水",
             '[1, 1, 0]': "泽",
             '[0, 1, 1]': "风",
             }

        # 按照六十四宫组合的六十四卦顺序
        self.gua64List1 = ['乾', '姤', '遁', '否', '观', '剥', '晋', '大有',
                   '坎', '节', '屯', '既济', '革', '丰', '明夷', '师',
                   '艮', '贲', '大畜', '损', '睽', '履', '中孚', '渐',
                   '震', '豫', '解', '恒', '升', '井', '大过', '随',
                   '巽', '小畜', '家人', '益', '无妄', '噬盍', '颐', '蛊',
                   '离', '旅', '鼎', '未济', '蒙', '涣', '讼', '天火同人',
                   '坤', '复', '临', '泰', '大壮', '夬', '需', '比',
                   '兑', '困', '萃', '咸', '蹇', '谦', '小过', '归妹']

        # 按照周易这本书的六十四卦顺序
        self.gua64list2=['乾', '坤', '屯', '蒙', '需', '讼', '师',
                       '比', '小畜', '履', '泰', '否', '同人', '大有',
                       '谦', '豫', '随', '蛊', '临', '观', '噬嗑',
                       '贲', '剥', '复', '无妄', '大畜', '颐', '大过',
                       '坎', '离', '咸', '恒', '遁', '大壮', '晋', '明夷',
                       '家人', '睽', '蹇', '解', '损', '益', '夬', '姤',
                       '萃', '升', '困', '井', '革', '鼎', '震', '艮', '渐',
                       '归妹', '丰', '旅', '巽', '兑', '涣', '节',
                       '中孚', '小过', '既济', '未济']

        # 根据六十四种组合收录的六十四卦字典
        self.table_64Gua = {('天', '天'): '乾',
                           ('天', '风'): '姤',
                           ('天', '山'): '遁',
                           ('天', '地'): '否',
                           ('风', '地'): '观',
                           ('山', '地'): '剥',
                           ('火', '地'): '晋',
                           ('火', '天'): '大有',
                           ('水', '水'): '坎',
                           ('水', '泽'): '节',
                           ('水', '雷'): '屯',
                           ('水', '火'): '既济',
                           ('泽', '火'): '革',
                           ('雷', '火'): '丰',
                           ('地', '火'): '明夷',
                           ('地', '水'): '师',
                           ('山', '山'): '艮',
                           ('山', '火'): '贲',
                           ('山', '天'): '大畜',
                           ('山', '泽'): '损',
                           ('火', '泽'): '睽',
                           ('天', '泽'): '履',
                           ('风', '泽'): '中孚',
                           ('风', '山'): '渐',
                           ('雷', '雷'): '震',
                           ('雷', '地'): '豫',
                           ('雷', '水'): '解',
                           ('雷', '风'): '恒',
                           ('地', '风'): '升',
                           ('水', '风'): '井',
                           ('泽', '风'): '大过',
                           ('泽', '雷'): '随',
                           ('风', '风'): '巽',
                           ('风', '天'): '小畜',
                           ('风', '火'): '家人',
                           ('风', '雷'): '益',
                           ('天', '雷'): '无妄',
                           ('火', '雷'): '噬盍',
                           ('山', '雷'): '颐',
                           ('山', '风'): '蛊',
                           ('火', '火'): '离',
                           ('火', '山'): '旅',
                           ('火', '风'): '鼎',
                           ('火', '水'): '未济',
                           ('山', '水'): '蒙',
                           ('风', '水'): '涣',
                           ('天', '水'): '讼',
                           ('天', '火'): '天火同人',
                           ('地', '地'): '坤',
                           ('地', '雷'): '复',
                           ('地', '泽'): '临',
                           ('地', '天'): '泰',
                           ('雷', '天'): '大壮',
                           ('泽', '天'): '夬',
                           ('水', '天'): '需',
                           ('水', '地'): '比',
                           ('泽', '泽'): '兑',
                           ('泽', '水'): '困',
                           ('泽', '地'): '萃',
                           ('泽', '山'): '咸',
                           ('水', '山'): '蹇',
                           ('地', '山'): '谦',
                           ('雷', '山'): '小过',
                           ('雷', '泽'): '归妹'
                           }

        # 读取六十四卦每一卦的信息
        with open('./周易卦象信息.json','r') as f:
            content=f.read()
            data=json.loads(content)
        self.table_info64Gua=data # 六十四卦全部信息


    def getYao(self,seed1=1,seed2=2,seed3=3,seed4=4):
        '''
        这个函数用来获取单独的一爻
        占卜方法：通过周易提到的哲草占卜法。
        :param seed1:
        :param seed2:
        :param seed3:
        :param seed4:
        :return:返回爻的数字，九八七六，四者之一
        '''


        '''                         初始化                        '''
        seedlist=[seed1,seed2,seed3,seed4]
        plant=50 # 大衍之数50
        zhecao=49 # 实际使用的哲草个数

        ''' ===================模拟"分而为二以象两"===================='''
        random.seed(seedlist[2])
        # 随机生成第一堆草的个数
        zhecao1=random.randint(1,48)
        zhecao2=zhecao-zhecao1

        '''========================模拟“挂一以象三”==========================='''
        random.seed(seedlist[3])
        #生成随机数，以确定选择从哪一堆草里拿出一根
        selectResult=random.randint(0,1)

        # 防止把某一堆的最后一根草拿掉
        if selectResult==0 & zhecao1 == 1:
            selectResult=1
        if selectResult==1 & zhecao2-1==0:
            selectResult=0

        # 拿出那根草
        if selectResult==0:
            zhecao1 -=1
        if selectResult==1:
            zhecao2 -=1

        # 这时一共有48根

        '''=======================模拟 数之以四以象四时=============================='''

        yu1=int(zhecao1)%4
        yu2=int(zhecao2)%4

        '''============================模拟 归奇于指以象闰=============================='''
        # print(yu1,yu2)
        if yu1==0:
            zhecao1 -=4
            zhecao2 -=4
        else:
            zhecao1 -=yu1
            zhecao2 -=yu2

        '''===============================对上述过程循环两次===================================='''
        i=0
        # 全部哲草的个数
        zhecao=zhecao1+zhecao2
        while i!=2:
            i+=1
            # print(zhecao)
            random.seed(seedlist[i])
            # 二分
            zhecao1=random.randint(1,zhecao-1)
            zhecao2=zhecao-zhecao1

            # 数之
            yu1=zhecao1%4
            yu2=int(zhecao2)%4

            # 归奇
            if yu1 ==0:
                zhecao1 -=4
                zhecao2 -=4
            else:
                zhecao1 -=yu1
                zhecao2 -=yu2

            zhecao=int(zhecao1+zhecao2)

        # 经过三变之后，哲草的个数有四种可能性：36，32，28，24

        # 获取占卜的爻
        yao=zhecao//4 # 9,8,7,6
        # 九七属于阳，八六属于阴。九为老阳，七为少阳。八为少阴，六为老阴。

        return yao


    def getGua(self,seedlist):
        '''
        用来生成一个卦
        :param seedlist: 随机数列表，需要有24个元素
        :return: 返回代表一个卦的01列表
        '''
        # seedlist=list(range(24))
        yaoList=[]
        for i in range(6):
            ii=int(i*4)

            # 获取每一爻的结果
            yaoList.append(self.getYao(seed1=seedlist[ii],seed2=seedlist[ii+1],
                                  seed3=seedlist[ii+2],seed4=seedlist[ii+3]))

        # 变爻
        newyaoList=deepcopy(yaoList)
        # 九和六是变爻，七和八是不变爻
        numYin2Yang=0
        numYang2Yin=0
        for x in range(len(yaoList)):
            if yaoList[x] == 9:
                numYang2Yin+=1
                newyaoList[x]=8
            if yaoList[x] ==6:
                newyaoList[x]=9
                numYin2Yang+=1

        # 形成阴阳
        yinyangList=[]
        # print(len(yaoList))
        for xx in range(len(yaoList)):
            if newyaoList[xx]==9 or newyaoList[xx]==7:
                yinyangList.append(1)
            else:
                yinyangList.append(0)

        # print(yinyangList)
        return yinyangList,numYin2Yang,numYang2Yin

    def query(self,yinyanglist):
        '''
        查询。通过输入一个卦，查询该卦的全部信息
        :param yinyanglist:01列表，用来表征一个卦的信息
        :return:卦的信息，字符串
        '''
        yyl1=str(yinyanglist[0:3])
        yyl2=str(yinyanglist[3:])


        gua_under=self.table_nameWith8Gua[yyl1]
        gua_upper=self.table_nameWith8Gua[yyl2]

        gua=self.table_64Gua[(gua_upper,gua_under)]

        info=self.table_info64Gua[gua]

        return gua_under,gua_upper,gua,info


    def suanGua(self):

        gualist,_,__=self.getGua(self.randomSeedlist)
        under,upper,gua,info=self.query(yinyanglist=gualist)
        print('测试结果生成中...')
        print('下{0}上{1},{1}{0}{2}，{2}卦'.format(under,upper,gua))
        print('======'*10+'详细信息'+'====='*10)
        print(info)


if __name__ =="__main__":
    test=ZhouYiGua(randomSeedlist='まるかいて地球 まるかいて地球')
    test.suanGua()







