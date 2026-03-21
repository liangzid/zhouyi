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
    showQuestionInfo: false,

    // 算卦方式
    divinationType: 'dayanshi',

    // 复制成功弹窗
    showCopySuccess: false
  },

  onLoad(options) {
    const { results, guaIndex, bianGuaIndex, questionInfo, divinationType } = options;

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
        showQuestionInfo: showQuestionInfo,
        divinationType: divinationType || 'dayanshi'
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

  // 生成分享图并保存到相册
  saveShareImage() {
    wx.showLoading({ title: '正在生成图片...' });

    const { guaDetail, bianGuaDetail, hasBian, questionInfo, divinationType, bianYaoIndices } = this.data;
    if (!guaDetail) {
      wx.hideLoading();
      return;
    }

    const methodName = divinationType === 'dayanshi' ? '大衍筮法' : '铜钱卦';
    const ctx = wx.createCanvasContext('shareCanvas');
    const W = 750; // canvas width in rpx
    const rpx = W / 750; // 转换比例

    // 颜色定义
    const colors = {
      bg: '#1a1a2e',
      bg2: '#16213e',
      gold: '#e8d5b7',
      goldDark: '#c9a86c',
      text: '#b0b0b0',
      textLight: '#e8d5b7',
      accent: '#c9a86c',
      gray: '#888888'
    };

    let y = 40; // 当前位置

    // 绘制背景
    ctx.setFillStyle(colors.bg);
    ctx.fillRect(0, 0, W, 1600);
    ctx.setFillStyle(colors.bg2);
    ctx.fillRect(0, 0, W, 1600);

    // 绘制渐变分隔线
    const drawGradientLine = (y) => {
      const gradient = ctx.createLinearGradient(80, 0, W - 80, 0);
      gradient.addColorStop(0, 'transparent');
      gradient.addColorStop(0.5, 'rgba(201, 168, 108, 0.5)');
      gradient.addColorStop(1, 'transparent');
      ctx.setStrokeStyle(gradient);
      ctx.setLineWidth(1);
      ctx.beginPath();
      ctx.moveTo(80, y);
      ctx.lineTo(W - 80, y);
      ctx.stroke();
    };

    // 文字换行绘制函数
    const drawText = (text, x, yPos, maxWidth, fontSize, color, lineHeight) => {
      ctx.setFontSize(fontSize);
      ctx.setFillStyle(color);
      ctx.textBaseline = 'top';

      const chars = text.split('');
      let line = '';
      let currentY = yPos;

      for (let i = 0; i < chars.length; i++) {
        const testLine = line + chars[i];
        const metrics = ctx.measureText(testLine);

        if (metrics.width > maxWidth && line.length > 0) {
          ctx.fillText(line, x, currentY);
          line = chars[i];
          currentY += lineHeight;
        } else {
          line = testLine;
        }
      }
      if (line.length > 0) {
        ctx.fillText(line, x, currentY);
      }

      return currentY + lineHeight; // 返回下一行 Y 位置
    };

    // 绘制标题
    ctx.setFontSize(44);
    ctx.setFillStyle(colors.gold);
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.fillText('周易算卦', W / 2, y + 30);
    y += 80;

    // 绘制副标题（占卜方法）
    ctx.setFontSize(24);
    ctx.setFillStyle(colors.gray);
    ctx.fillText(`占卜方法：${methodName}`, W / 2, y);
    y += 50;

    drawGradientLine(y);
    y += 30;

    // 绘制问事信息区块
    ctx.setFontSize(28);
    ctx.setFillStyle(colors.goldDark);
    ctx.textAlign = 'left';
    ctx.fillText('本次问事', 50, y);
    y += 45;

    const qInfoItems = [
      { label: '所问何事', value: questionInfo?.event || '未填写' },
      { label: '所在地点', value: questionInfo?.locationText || '未填写' },
      { label: '占卜时间', value: questionInfo?.currentTime || '未填写' },
      { label: '事件发生', value: questionInfo?.targetTime || '未填写' }
    ];

    for (const item of qInfoItems) {
      ctx.setFontSize(22);
      ctx.setFillStyle(colors.gray);
      ctx.fillText(item.label + '：', 50, y);
      const labelWidth = ctx.measureText(item.label + '：').width;
      ctx.setFillStyle(colors.text);
      ctx.fillText(item.value, 50 + labelWidth, y);
      y += 38;
    }

    y += 20;
    drawGradientLine(y);
    y += 40;

    // 绘制卦象区块
    const guaName = hasBian && bianGuaDetail
      ? `${guaDetail.guaName}卦 → ${bianGuaDetail.guaName}卦`
      : `${guaDetail.guaName}卦`;

    ctx.setFontSize(40);
    ctx.setFillStyle(colors.gold);
    ctx.textAlign = 'center';
    ctx.fillText(guaName, W / 2, y);
    y += 60;

    // 绘制变爻位置
    if (hasBian && bianYaoIndices.length > 0) {
      const bianYaoNames = bianYaoIndices.map(i => ['初爻', '二爻', '三爻', '四爻', '五爻', '上爻'][i]).join('、');
      ctx.setFontSize(20);
      ctx.setFillStyle('#ff6b6b');
      ctx.fillText(`变爻：${bianYaoNames}`, W / 2, y);
      y += 35;
    }

    y += 20;

    // 绘制本卦信息
    const drawGuaSection = (detail, title, startY) => {
      let curY = startY;

      if (hasBian && bianGuaDetail) {
        ctx.setFontSize(26);
        ctx.setFillStyle(colors.accent);
        ctx.textAlign = 'center';
        ctx.fillText(title, W / 2, curY);
        curY += 40;
      }

      // 卦辞
      curY = drawText('卦辞：' + detail.guaCi, 50, curY, W - 100, 22, colors.textLight, 34);

      // 彖传
      curY = drawText('彖传：' + detail.duan, 50, curY, W - 100, 20, colors.text, 30);

      // 大象传
      curY = drawText('大象传：' + detail.xiang, 50, curY, W - 100, 20, colors.text, 30);

      // 爻辞
      const yaoNames = ['初爻', '二爻', '三爻', '四爻', '五爻', '上爻'];
      for (let i = 0; i < detail.yaoCi.length; i++) {
        curY = drawText(`${yaoNames[i]}：${detail.yaoCi[i]}`, 50, curY, W - 100, 18, colors.gray, 28);
      }

      return curY;
    };

    y = drawGuaSection(guaDetail, `【本卦 ${guaDetail.guaName}】`, y);

    // 绘制变卦信息
    if (hasBian && bianGuaDetail) {
      y += 30;
      drawGradientLine(y);
      y += 30;
      y = drawGuaSection(bianGuaDetail, `【变卦 ${bianGuaDetail.guaName}】`, y);
    }

    // 绘制底部信息
    y += 30;
    drawGradientLine(y);
    y += 25;
    ctx.setFontSize(18);
    ctx.setFillStyle(colors.gray);
    ctx.textAlign = 'center';
    ctx.fillText('由周易算卦小程序生成', W / 2, y);

    // 执行绘制
    ctx.draw(false, () => {
      // 导出为图片
      wx.canvasToTempFilePath({
        canvasId: 'shareCanvas',
        success: (res) => {
          // 保存到相册
          wx.saveImageToPhotosAlbum({
            filePath: res.tempFilePath,
            success: () => {
              wx.hideLoading();
              wx.showToast({
                title: '已保存到相册',
                icon: 'success'
              });
            },
            fail: (err) => {
              wx.hideLoading();
              console.error('保存失败', err);
              wx.showToast({
                title: '保存失败',
                icon: 'none'
              });
            }
          });
        },
        fail: (err) => {
          wx.hideLoading();
          console.error('生成图片失败', err);
          wx.showToast({
            title: '生成失败',
            icon: 'none'
          });
        }
      });
    });
  },

  // 复制完整卦象信息
  copyGuaCi() {
    const { guaDetail, bianGuaDetail, hasBian, questionInfo, divinationType, bianYaoIndices } = this.data;
    if (!guaDetail) return;

    // 格式化占卜方法
    const methodName = divinationType === 'dayanshi' ? '大衍筮法' : '铜钱卦';

    // 拼接爻辞
    const formatYaoCi = (detail) => {
      return detail.yaoCi.map((ci, i) => `${['初爻', '二爻', '三爻', '四爻', '五爻', '上爻'][i]}：${ci}`).join('；');
    };

    // 变爻位置描述
    const bianYaoDesc = bianYaoIndices.length > 0
      ? `变爻位置：${bianYaoIndices.map(i => ['初爻', '二爻', '三爻', '四爻', '五爻', '上爻'][i]).join('、')}`
      : '';

    // 构建完整信息
    const parts = [
      `用户使用${methodName}进行占卜，`,
      `所问：${questionInfo?.event || '未填写'}，`,
      `占卜地点：${questionInfo?.locationText || '未填写'}，`,
      `占卜时间：${questionInfo?.currentTime || '未填写'}，`,
      `事件发生时间：${questionInfo?.targetTime || '未填写'}，`,
      hasBian && bianGuaDetail
        ? `所得卦象为本卦${guaDetail.guaName}卦、变卦${bianGuaDetail.guaName}卦。`
        : `所得卦象为${guaDetail.guaName}卦。`
    ];

    // 本卦信息
    parts.push(`【本卦 ${guaDetail.guaName}】`);
    parts.push(`卦辞：${guaDetail.guaCi}`);
    parts.push(`彖传：${guaDetail.duan}`);
    parts.push(`大象传：${guaDetail.xiang}`);
    parts.push(`爻辞：${formatYaoCi(guaDetail)}`);

    // 变卦信息
    if (hasBian && bianGuaDetail) {
      parts.push(`【变卦 ${bianGuaDetail.guaName}】`);
      parts.push(`卦辞：${bianGuaDetail.guaCi}`);
      parts.push(`彖传：${bianGuaDetail.duan}`);
      parts.push(`大象传：${bianGuaDetail.xiang}`);
      parts.push(`爻辞：${formatYaoCi(bianGuaDetail)}`);
      if (bianYaoDesc) {
        parts.push(bianYaoDesc);
      }
    }

    wx.setClipboardData({
      data: parts.join('\n'),
      success: () => {
        this.setData({ showCopySuccess: true });
      }
    });
  },

  // 关闭复制成功弹窗
  closeCopySuccess() {
    this.setData({ showCopySuccess: false });
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
