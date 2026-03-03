// pages/question/question.js
const divination = require('../../utils/divination.js');
const guaData = require('../../utils/gua_data.js');

Page({
  data: {
    currentStep: 1,        // 当前步骤：1-问事，2-地址，3-时间，4-目标时间
    totalSteps: 4,

    // 步骤1 - 问事
    event: '',
    eventInput: '',
    quickEvents: ['事业', '财运', '婚姻', '学业', '健康', '出行', '投资', '其他'],

    // 步骤2 - 地址
    location: null,
    locationText: '',
    locationLoading: false,

    // 步骤3 - 时间
    currentTime: '',
    useCurrentTime: true,

    // 步骤4 - 目标时间
    targetTime: '',

    // 算卦方式
    divinationType: 'dayanshi',

    // 动画状态
    isDivinating: false,
    animationFrame: 0
  },

  onLoad(options) {
    // 获取算卦方式
    if (options.divinationType) {
      this.setData({ divinationType: options.divinationType });
    }

    // 设置当前时间
    this.setCurrentTime();
  },

  // 设置当前时间
  setCurrentTime() {
    const now = new Date();
    const year = now.getFullYear();
    const month = String(now.getMonth() + 1).padStart(2, '0');
    const day = String(now.getDate()).padStart(2, '0');
    const hour = String(now.getHours()).padStart(2, '0');
    const minute = String(now.getMinutes()).padStart(2, '0');
    const currentTime = `${year}-${month}-${day} ${hour}:${minute}`;
    this.setData({ currentTime });
  },

  // ===== 步骤1：问事 =====

  // 输入事件
  onEventInput(e) {
    this.setData({ eventInput: e.detail.value });
  },

  // 选择快捷问题
  selectQuickEvent(e) {
    const event = e.currentTarget.dataset.event;
    this.setData({
      event: event,
      eventInput: event
    });
  },

  // 确认事件，进入下一步
  confirmEvent() {
    const event = this.data.eventInput.trim();
    if (!event) {
      wx.showToast({
        title: '请输入想问的事情',
        icon: 'none'
      });
      return;
    }
    this.setData({ event: event });
    this.nextStep();
  },

  // ===== 步骤2：地址 =====

  // 获取当前位置
  getLocation() {
    this.setData({ locationLoading: true });

    wx.getLocation({
      type: 'wgs84',
      success: (res) => {
        const location = {
          latitude: res.latitude,
          longitude: res.longitude
        };
        // 简单显示经纬度
        const locationText = `经度: ${res.latitude.toFixed(4)}, 纬度: ${res.longitude.toFixed(4)}`;
        this.setData({
          location: location,
          locationText: locationText,
          locationLoading: false
        });
      },
      fail: (err) => {
        console.log('获取位置失败', err);
        this.setData({ locationLoading: false });
        wx.showToast({
          title: '无法获取位置',
          icon: 'none'
        });
      }
    });
  },

  // 跳过地址，进入下一步
  skipLocation() {
    this.setData({
      location: null,
      locationText: ''
    });
    this.nextStep();
  },

  // ===== 步骤3：时间 =====

  // 切换是否使用当前时间
  toggleTime() {
    this.setData({ useCurrentTime: !this.data.useCurrentTime });
  },

  // 确认时间，进入下一步
  confirmTime() {
    this.nextStep();
  },

  // ===== 步骤4：目标时间 =====

  // 输入目标时间
  onTargetTimeInput(e) {
    this.setData({ targetTime: e.detail.value });
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
    // 准备问事信息（在算卦之前准备好，用于设置种子）
    const questionInfo = {
      event: this.data.event,
      location: this.data.location,
      locationText: this.data.locationText,
      currentTime: this.data.useCurrentTime ? this.data.currentTime : null,
      targetTime: this.data.targetTime || null
    };

    console.log('=== DEBUG: 问事信息 ===');
    console.log('event:', questionInfo.event);
    console.log('location:', questionInfo.location);
    console.log('locationText:', questionInfo.locationText);
    console.log('currentTime:', questionInfo.currentTime);
    console.log('targetTime:', questionInfo.targetTime);

    // 使用问事信息设置随机数种子
    divination.setQuestionSeed(questionInfo);

    let results;

    if (this.data.divinationType === 'dayanshi') {
      results = divination.dayanshiDivinate();
    } else {
      results = divination.coinDivinate();
    }

    console.log('=== DEBUG: 爻结果 ===');
    console.log(results);

    // 计算卦象
    const guaInfo = divination.calculateGua(results);

    // 使用卦索引查找卦象详细信息
    const guaDetail = guaData.getGuaByIndex(guaInfo.guaIndex);

    // 计算变卦
    const bianInfo = divination.calculateBianGua(results);
    let bianGuaDetail = null;
    if (bianInfo.hasBian) {
      bianGuaDetail = guaData.getGuaByIndex(bianInfo.bianGuaIndex);
    }

    // 跳转到结果页，传递问事信息
    const encodedQuestionInfo = encodeURIComponent(JSON.stringify(questionInfo));
    wx.navigateTo({
      url: `/pages/result/result?results=${encodeURIComponent(JSON.stringify(results))}&guaIndex=${guaInfo.guaIndex}&bianGuaIndex=${bianInfo.hasBian ? bianInfo.bianGuaIndex : -1}&questionInfo=${encodedQuestionInfo}`
    });

    this.setData({ isDivinating: false, animationFrame: 0 });
  },

  // ===== 通用 =====

  // 进入下一步
  nextStep() {
    if (this.data.currentStep < this.data.totalSteps) {
      this.setData({ currentStep: this.data.currentStep + 1 });
    }
  },

  // 返回上一步
  prevStep() {
    if (this.data.currentStep > 1) {
      this.setData({ currentStep: this.data.currentStep - 1 });
    } else {
      wx.navigateBack();
    }
  }
})
