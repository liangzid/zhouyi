// app.js
App({
  globalData: {
    userInfo: null,
    isLoggedIn: false,
    openid: null
  },

  onLaunch() {
    // 小程序启动逻辑
    // 检查本地存储的登录状态
    this.checkLoginStatus();
  },

  // 检查登录状态
  checkLoginStatus() {
    const userInfo = wx.getStorageSync('userInfo');
    const openid = wx.getStorageSync('openid');
    if (userInfo && openid) {
      this.globalData.userInfo = userInfo;
      this.globalData.isLoggedIn = true;
      this.globalData.openid = openid;
    }
  },

  // 微信登录
  login(callback) {
    const that = this;

    // 先检查是否有存储的用户信息
    const storedUserInfo = wx.getStorageSync('userInfo');
    const storedOpenid = wx.getStorageSync('openid');

    if (storedUserInfo && storedOpenid) {
      // 已有登录信息
      this.globalData.userInfo = storedUserInfo;
      this.globalData.isLoggedIn = true;
      this.globalData.openid = storedOpenid;
      if (callback) callback(storedUserInfo);
      return;
    }

    // 调用 wx.login 获取 code
    wx.login({
      success: res => {
        if (res.code) {
          // 登录成功
          // 注意：实际生产环境需要将 code 发送到后端获取 openid
          // 这里使用本地模拟
          const openid = 'wx_' + res.code;
          that.globalData.openid = openid;
          wx.setStorageSync('openid', openid);

          // 创建模拟用户信息（因为 getUserProfile 在开发工具中有诸多限制）
          const mockUserInfo = {
            nickName: '用户_' + res.code.substring(0, 6),
            avatarUrl: ''
          };
          that.globalData.userInfo = mockUserInfo;
          that.globalData.isLoggedIn = true;
          wx.setStorageSync('userInfo', mockUserInfo);

          if (callback) callback(mockUserInfo);
        }
      },
      fail: err => {
        console.error('wx.login 失败', err);
        if (callback) callback(null);
      }
    });
  },

  // 退出登录
  logout() {
    this.globalData.userInfo = null;
    this.globalData.isLoggedIn = false;
    this.globalData.openid = null;
    wx.removeStorageSync('userInfo');
    wx.removeStorageSync('openid');
  },

  // 显示隐私提示
  showPrivacyModal(confirmCallback, cancelCallback) {
    wx.showModal({
      title: '温馨提示',
      content: '您当前未登录，算卦结果将不会被保存。我们高度重视您的隐私保护，您填写的问事信息仅用于本次算卦，不会被用于任何其他用途。',
      confirmText: '继续算卦',
      cancelText: '登录后再算',
      success: res => {
        if (res.confirm) {
          if (confirmCallback) confirmCallback();
        } else {
          if (cancelCallback) cancelCallback();
        }
      }
    });
  }
})
