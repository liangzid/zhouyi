// pages/index/index.js
const divination = require('../../utils/divination.js');
const guaData = require('../../utils/gua_data.js');

Page({
  data: {
    divinationType: 'dayanshi', // 'dayanshi' 大衍筮法, 'coin' 铜钱卦
    isDivinating: false,
    animationFrame: 0
  },

  onLoad() {
    // 页面加载
  },

  // 选择算卦方式
  selectType(e) {
    const type = e.currentTarget.dataset.type;
    this.setData({ divinationType: type });
  },

  // 开始算卦
  startDivination() {
    if (this.data.isDivinating) return;

    this.setData({ isDivinating: true });

    // 模拟摇卦动画
    let frame = 0;
    const maxFrames = 30;

    const animate = () => {
      frame++;
      this.setData({ animationFrame: frame });

      if (frame < maxFrames) {
        setTimeout(animate, 80);
      } else {
        this.performDivination();
      }
    };

    animate();
  },

  // 执行算卦
  performDivination() {
    let results;

    if (this.data.divinationType === 'dayanshi') {
      results = divination.dayanshiDivinate();
    } else {
      results = divination.coinDivinate();
    }

    // 计算卦象
    const guaInfo = divination.calculateGua(results);

    // 获取卦象详细信息
    const guaDetail = guaData.getGuaByIndex(guaInfo.guaIndex);

    // 计算变卦
    const bianInfo = divination.calculateBianGua(results);
    let bianGuaDetail = null;
    if (bianInfo.hasBian) {
      bianGuaDetail = guaData.getGuaByIndex(bianInfo.bianGuaIndex);
    }

    // 跳转到结果页
    wx.navigateTo({
      url: `/pages/result/result?results=${encodeURIComponent(JSON.stringify(results))}&guaIndex=${guaInfo.guaIndex}&bianGuaIndex=${bianInfo.hasBian ? bianInfo.bianGuaIndex : -1}`
    });

    this.setData({ isDivinating: false, animationFrame: 0 });
  }
})
