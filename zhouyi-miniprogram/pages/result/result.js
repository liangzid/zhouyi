// pages/result/result.js
const guaData = require('../../utils/gua_data.js');

Page({
  data: {
    results: [],
    baGua: guaData.baGua,
    guaDetail: null,
    bianGuaDetail: null,
    hasBian: false,
    currentTab: 0, // 0: 本卦, 1: 变卦
    yaoTypes: ['初爻', '二爻', '三爻', '四爻', '五爻', '上爻']
  },

  onLoad(options) {
    const { results, guaIndex, bianGuaIndex } = options;

    try {
      const resultsArr = JSON.parse(decodeURIComponent(results));
      const guaIndexNum = parseInt(guaIndex);
      const bianGuaIndexNum = parseInt(bianGuaIndex);

      const guaDetail = guaData.getGuaByIndex(guaIndexNum);

      let bianGuaDetail = null;
      let hasBian = false;

      if (bianGuaIndexNum >= 0) {
        bianGuaDetail = guaData.getGuaByIndex(bianGuaIndexNum);
        hasBian = true;
      }

      // 检查是否有变爻
      const hasBianYao = resultsArr.some(r => r.type === '老阳' || r.type === '老阴');

      this.setData({
        results: resultsArr,
        guaDetail,
        bianGuaDetail,
        hasBian: hasBian && hasBianYao
      });

      // 设置导航栏标题
      wx.setNavigationBarTitle({
        title: `${guaDetail.guaName}卦`
      });
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
    this.setData({ currentTab: index });
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
