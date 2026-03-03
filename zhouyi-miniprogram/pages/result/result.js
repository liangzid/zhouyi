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
          showQuestionInfo = !!qInfo.event;
        } catch (e) {
          console.log('解析问事信息失败', e);
        }
      }

      // 使用索引查找卦象信息
      const guaDetail = guaData.getGuaByIndex(guaIndexNum);

      console.log('=== DEBUG result: guaDetail ===', guaDetail ? guaDetail.guaName : 'null');

      let bianGuaDetail = null;
      let hasBian = false;

      if (!isNaN(bianGuaIndexNum) && bianGuaIndexNum >= 0) {
        bianGuaDetail = guaData.getGuaByIndex(bianGuaIndexNum);
        hasBian = !!bianGuaDetail;
      }

      // 交换六爻顺序：初↔四、二↔五、三↔上
      const swapYaoOrder = (arr) => [
        arr[3], arr[4], arr[5],
        arr[0], arr[1], arr[2]
      ];

      // 保存原始六爻数据（交换后）
      const originalResults = swapYaoOrder(resultsArr);

      // 计算变卦的六爻数据
      let bianYaoResults = [];
      if (hasBian) {
        const bianInfo = divination.calculateBianGua(resultsArr);
        if (bianInfo.bianYaoResults) {
          bianYaoResults = swapYaoOrder(bianInfo.bianYaoResults);
        }
      }

      this.setData({
        results: originalResults,
        originalResults,
        bianYaoResults,
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
    const { originalResults, bianYaoResults, bianGuaDetail, guaDetail } = this.data;

    // 根据 tab 切换六爻数据和标题
    if (index === 1 && bianYaoResults.length > 0) {
      this.setData({
        currentTab: index,
        results: bianYaoResults
      });
      wx.setNavigationBarTitle({
        title: `${bianGuaDetail.guaName}卦`
      });
    } else {
      this.setData({
        currentTab: index,
        results: originalResults
      });
      wx.setNavigationBarTitle({
        title: `${guaDetail.guaName}卦`
      });
    }
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
