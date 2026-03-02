// pages/index/index.js
const divination = require('../../utils/divination.js');
const guaData = require('../../utils/gua_data.js');

const app = getApp();

Page({
  data: {
    divinationType: 'dayanshi', // 'dayanshi' 大衍筮法, 'coin' 铜钱卦
    isDivinating: false,
    animationFrame: 0,
    userInfo: null,
    isLoggedIn: false
  },

  onLoad() {
    // 页面加载
  },

  onShow() {
    // 每次显示页面时检查登录状态
    this.checkLoginStatus();
  },

  // 检查登录状态
  checkLoginStatus() {
    const userInfo = wx.getStorageSync('userInfo');
    const isLoggedIn = !!userInfo;
    this.setData({
      userInfo: userInfo,
      isLoggedIn: isLoggedIn
    });
  },

  // 选择算卦方式
  selectType(e) {
    const type = e.currentTarget.dataset.type;
    this.setData({ divinationType: type });
  },

  // 微信登录
  login() {
    app.login((userInfo) => {
      if (userInfo) {
        this.setData({
          userInfo: userInfo,
          isLoggedIn: true
        });
        wx.showToast({
          title: '登录成功',
          icon: 'success'
        });
      } else {
        wx.showToast({
          title: '您可以使用访客模式',
          icon: 'none'
        });
      }
    });
  },

  // 开始算卦
  startDivination() {
    console.log('startDivination called, isDivinating:', this.data.isDivinating);
    if (this.data.isDivinating) return;

    const that = this;

    // 检查是否已登录
    if (!this.data.isLoggedIn) {
      // 未登录，显示隐私提示
      wx.showModal({
        title: '温馨提示',
        content: '您当前未登录，算卦结果将不会被保存。我们高度重视您的隐私保护，您填写的问事信息仅用于本次算卦，不会被用于任何其他用途。',
        confirmText: '继续算卦',
        cancelText: '登录后再算',
        success: function(res) {
          console.log('Modal result:', res);
          if (res.confirm) {
            // 用户选择继续算卦
            that.goToQuestion();
          } else if (res.cancel) {
            // 用户选择先登录
            that.login();
          }
        }
      });
    } else {
      // 已登录，直接进入问事流程
      this.goToQuestion();
    }
  },

  // 跳转到问事页面
  goToQuestion() {
    // 将算卦方式传递给问事页面
    wx.navigateTo({
      url: `/pages/question/question?divinationType=${this.data.divinationType}`
    });
  },

  // 执行算卦（保留原逻辑，用于动画后调用）
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
