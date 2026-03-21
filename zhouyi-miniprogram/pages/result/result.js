// pages/result/result.js
const guaData = require('../../utils/gua_data.js');
const divination = require('../../utils/divination.js');

Page({
  data: {
    results: [],
    baGua: guaData.baGua,
    guaDetail: null,
    bianGuaDetail: null,
    hasBian: false,
    currentTab: 0, // 0: 本卦, 1: 变卦
    yaoTypes: ['初爻', '二爻', '三爻', '四爻', '五爻', '上爻'],

    // 上卦（三、四、五爻）和下卦（初、二、三爻）
    upperYaoResults: [],  // 上卦爻
    lowerYaoResults: [],  // 下卦爻
    upperYaoResultsBian: [], // 变卦上卦爻
    lowerYaoResultsBian: [], // 变卦下卦爻
    bianYaoIndices: [], // 变爻索引（原数组顺序：0-5对应初到上）

    // 爻辞弹窗
    showYaoPopup: false,
    selectedYaoIndex: 0,
    selectedYaoType: '',
    selectedYaoYao: 1,

    // 问事信息
    questionInfo: null,
    showQuestionInfo: false
  },

  onLoad(options) {
    const { results, guaIndex, bianGuaIndex, questionInfo } = options;

    try {
      const resultsArr = JSON.parse(decodeURIComponent(results));
      const guaIndexNum = parseInt(guaIndex);
      const bianGuaIndexNum = parseInt(bianGuaIndex);

      console.log('=== DEBUG result: guaIndex ===', guaIndexNum);
      console.log('=== DEBUG result: bianGuaIndex ===', bianGuaIndexNum);

      // 解析问事信息
      let qInfo = null;
      let showQuestionInfo = false;
      if (questionInfo) {
        try {
          qInfo = JSON.parse(decodeURIComponent(questionInfo));
          showQuestionInfo = !!qInfo;
        } catch (e) {
          console.log('解析问事信息失败', e);
        }
      }

      // 使用索引查找卦象信息
      const guaDetail = guaData.getGuaByIndex(guaIndexNum);

      console.log('=== DEBUG result: guaDetail ===', guaDetail ? guaDetail.guaName : 'null');

      let bianGuaDetail = null;
      let hasBian = false;
      let bianYaoIndices = [];

      if (!isNaN(bianGuaIndexNum) && bianGuaIndexNum >= 0) {
        bianGuaDetail = guaData.getGuaByIndex(bianGuaIndexNum);
        hasBian = !!bianGuaDetail;
      }

      // 交换六爻顺序：初↔四、二↔五、三↔上
      // 原始顺序：resultsArr[0]=初, [1]=二, [2]=三, [3]=四, [4]=五, [5]=上
      // 交换后：[0]=四, [1]=五, [2]=上, [3]=初, [4]=二, [5]=三
      const swapYaoOrder = (arr) => [
        arr[3], arr[4], arr[5],
        arr[0], arr[1], arr[2]
      ];

      // 交换后的结果
      const swappedResults = swapYaoOrder(resultsArr);

      // 分离上卦（四、五、上）和下卦（初、二、三）
      // 交换后索引：0=四, 1=五, 2=上（上卦），3=初, 4=二, 5=三（下卦）
      const upperYaoResults = [swappedResults[0], swappedResults[1], swappedResults[2]]; // 四、五、上
      const lowerYaoResults = [swappedResults[3], swappedResults[4], swappedResults[5]]; // 初、二、三

      // 计算变卦的六爻数据
      let upperYaoResultsBian = [];
      let lowerYaoResultsBian = [];
      if (hasBian) {
        const bianInfo = divination.calculateBianGua(resultsArr);
        bianYaoIndices = bianInfo.bianYaoIndices || [];

        if (bianInfo.bianYaoResults) {
          const swappedBianResults = swapYaoOrder(bianInfo.bianYaoResults);
          upperYaoResultsBian = [swappedBianResults[0], swappedBianResults[1], swappedBianResults[2]];
          lowerYaoResultsBian = [swappedBianResults[3], swappedBianResults[4], swappedBianResults[5]];
        }
      }

      // 判断哪些位置有变爻（用于显示）
      // bianYaoIndices 是原始顺序 [0-5对应初到上]
      // 交换后：upper需要判断 3,4,5（对应原 3,4,5即四五六），lower需要判断 0,1,2（对应原 0,1,2即初二三）
      const getBianForPosition = (originalIndex) => bianYaoIndices.includes(originalIndex);
      const upperBianFlags = [getBianForPosition(3), getBianForPosition(4), getBianForPosition(5)];
      const lowerBianFlags = [getBianForPosition(0), getBianForPosition(1), getBianForPosition(2)];

      this.setData({
        results: swappedResults,
        originalResults: swappedResults,
        upperYaoResults,
        lowerYaoResults,
        upperYaoResultsBian,
        lowerYaoResultsBian,
        bianYaoIndices,
        guaDetail,
        bianGuaDetail,
        hasBian,
        questionInfo: qInfo,
        showQuestionInfo: showQuestionInfo
      });

      // 设置导航栏标题
      if (guaDetail) {
        wx.setNavigationBarTitle({
          title: `${guaDetail.guaName}卦`
        });
      }
    } catch (e) {
      console.error('解析结果失败:', e);
      wx.showToast({
        title: '数据解析失败',
        icon: 'none'
      });
    }
  },

  // 切换 tab
  switchTab(e) {
    const index = parseInt(e.currentTarget.dataset.index);
    const { originalResults, bianGuaDetail, guaDetail, upperYaoResults, lowerYaoResults, upperYaoResultsBian, lowerYaoResultsBian, hasBian } = this.data;

    // 根据 tab 切换六爻数据和标题
    if (index === 1 && hasBian && bianGuaDetail) {
      this.setData({
        currentTab: index,
        results: originalResults,
        upperYaoResults: upperYaoResultsBian,
        lowerYaoResults: lowerYaoResultsBian
      });
      wx.setNavigationBarTitle({
        title: `${bianGuaDetail.guaName}卦`
      });
    } else {
      this.setData({
        currentTab: index,
        results: originalResults,
        upperYaoResults: upperYaoResults,
        lowerYaoResults: lowerYaoResults
      });
      wx.setNavigationBarTitle({
        title: `${guaDetail.guaName}卦`
      });
    }
  },

  // 显示爻辞弹窗
  showYaoCi(e) {
    const { index, type, yao } = e.currentTarget.dataset;
    this.setData({
      showYaoPopup: true,
      selectedYaoIndex: index,
      selectedYaoType: type,
      selectedYaoYao: yao
    });
  },

  // 关闭爻辞弹窗
  closeYaoPopup() {
    this.setData({
      showYaoPopup: false
    });
  },

  // 复制卦辞
  copyGuaCi() {
    const detail = this.data.currentTab === 0 ? this.data.guaDetail : this.data.bianGuaDetail;
    if (!detail) return;

    const text = `【${detail.guaName}卦】\n\n卦辞：${detail.guaCi}\n\n彖传：${detail.duan}\n\n大象：${detail.xiang}`;

    wx.setClipboardData({
      data: text,
      success: () => {
        wx.showToast({
          title: '已复制',
          icon: 'success'
        });
      }
    });
  },

  // 分享
  onShareAppMessage() {
    const detail = this.data.guaDetail;
    return {
      title: `我算到了${detail.guaName}卦`,
      path: '/pages/index/index'
    };
  }
})
