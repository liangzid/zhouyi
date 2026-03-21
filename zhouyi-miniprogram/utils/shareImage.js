/**
 * 生成分享图的核心逻辑
 * 将卦象信息绘制到 Canvas 并导出为图片
 */

/**
 * 生成包含卦象信息的分享图
 * @param {Object} options - 分享图参数
 * @param {Object} options.guaDetail - 本卦信息
 * @param {Object} options.bianGuaDetail - 变卦信息
 * @param {Boolean} options.hasBian - 是否有变卦
 * @param {Array} options.bianYaoIndices - 变爻索引
 * @param {Object} options.questionInfo - 问事信息
 * @param {String} options.divinationType - 占卜方法
 * @returns {Promise<string>} - 返回生成的图片临时路径
 */
const generateShareImage = (options) => {
  const { guaDetail, bianGuaDetail, hasBian, bianYaoIndices, questionInfo, divinationType } = options;

  const methodName = divinationType === 'dayanshi' ? '大衍筮法' : '铜钱卦';
  const ctx = wx.createCanvasContext('shareCanvas');

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

  // 坐标设置
  const LEFT = 40;
  const RIGHT = 710;
  const W = RIGHT - LEFT; // 内容宽度 670
  let y = 25;

  // 估算中文字符宽度
  const getTextWidth = (text, fontSize) => {
    let width = 0;
    for (let i = 0; i < text.length; i++) {
      const char = text[i];
      if ('初三四五六爻卦辞彖传象大易算卜所地点时间生事件未填写·——'.includes(char)) {
        width += fontSize * 0.9;
      } else if ('。，、；：'.includes(char)) {
        width += fontSize * 0.4;
      } else if (char === ' ' || char === '→') {
        width += fontSize * 0.5;
      } else {
        width += fontSize;
      }
    }
    return width;
  };

  // 文字换行绘制
  const drawText = (text, x, yPos, maxWidth, fontSize, color, lineHeight) => {
    ctx.setFontSize(fontSize);
    ctx.setFillStyle(color);
    ctx.textBaseline = 'top';

    let currentY = yPos;
    let line = '';

    for (let i = 0; i < text.length; i++) {
      const char = text[i];
      const testLine = line + char;
      const testWidth = getTextWidth(testLine, fontSize);

      if (testWidth > maxWidth && line.length > 0) {
        ctx.fillText(line, x, currentY);
        line = char;
        currentY += lineHeight;
      } else {
        line = testLine;
      }
    }
    if (line.length > 0) {
      ctx.fillText(line, x, currentY);
    }
    return currentY + lineHeight;
  };

  // 按句号分段绘制文本
  const drawTextBySentence = (text, x, yPos, maxWidth, fontSize, color, lineHeight) => {
    const sentences = text.split(/(?<=[。；])/);
    let currentY = yPos;
    for (const sentence of sentences) {
      if (sentence.trim()) {
        currentY = drawText(sentence, x, currentY, maxWidth, fontSize, color, lineHeight);
      }
    }
    return currentY;
  };

  // 绘制分隔线
  const drawLine = (yPos) => {
    ctx.setStrokeStyle('rgba(201, 168, 108, 0.4)');
    ctx.setLineWidth(1);
    ctx.beginPath();
    ctx.moveTo(LEFT, yPos);
    ctx.lineTo(RIGHT, yPos);
    ctx.stroke();
  };

  // 绘制标题 - 左对齐
  ctx.setFontSize(26);
  ctx.setFillStyle(colors.gold);
  ctx.textAlign = 'left';
  ctx.textBaseline = 'middle';
  ctx.fillText('周易算卦', LEFT, y + 18);
  y += 45;

  // 副标题 - 左对齐
  ctx.setFontSize(15);
  ctx.setFillStyle(colors.gray);
  ctx.fillText('占卜方法：' + methodName, LEFT, y);
  y += 30;
  drawLine(y);
  y += 20;

  // 问事信息 - 左对齐
  ctx.setFontSize(18);
  ctx.setFillStyle(colors.goldDark);
  ctx.fillText('本次问事', LEFT, y);
  y += 30;

  const qInfoItems = [
    { label: '所问何事', value: questionInfo?.event || '未填写' },
    { label: '所在地点', value: questionInfo?.locationText || '未填写' },
    { label: '占卜时间', value: questionInfo?.currentTime || '未填写' },
    { label: '事件发生', value: questionInfo?.targetTime || '未填写' }
  ];

  for (const item of qInfoItems) {
    ctx.setFontSize(13);
    ctx.setFillStyle(colors.gray);
    ctx.fillText(item.label + '：' + item.value, LEFT, y, W);
    y += 24;
  }

  y += 14;
  drawLine(y);
  y += 20;

  // 卦象名称 - 左对齐
  const guaName = hasBian && bianGuaDetail
    ? guaDetail.guaName + '卦 → ' + bianGuaDetail.guaName + '卦'
    : guaDetail.guaName + '卦';

  ctx.setFontSize(22);
  ctx.setFillStyle(colors.gold);
  ctx.fillText(guaName, LEFT, y);
  y += 40;

  // 变爻位置
  if (hasBian && bianYaoIndices && bianYaoIndices.length > 0) {
    const bianYaoNames = bianYaoIndices.map(i => ['初爻', '二爻', '三爻', '四爻', '五爻', '上爻'][i]).join('、');
    ctx.setFontSize(11);
    ctx.setFillStyle('#ff6b6b');
    ctx.fillText('变爻：' + bianYaoNames, LEFT, y);
    y += 20;
  }

  y += 10;

  // 绘制卦象区块
  const drawGuaSection = (detail, title, startY, showTitle) => {
    let curY = startY;

    if (showTitle) {
      ctx.setFontSize(16);
      ctx.setFillStyle(colors.accent);
      ctx.fillText(title, LEFT, curY);
      curY += 26;
    }

    curY = drawText('卦辞：', LEFT, curY, W, 13, colors.textLight, 20);
    curY = drawTextBySentence(detail.guaCi, LEFT, curY, W, 12, colors.textLight, 18);

    curY = drawText('彖传：', LEFT, curY, W, 12, colors.text, 18);
    curY = drawTextBySentence(detail.duan, LEFT, curY, W, 11, colors.text, 16);

    curY = drawText('大象传：', LEFT, curY, W, 12, colors.text, 18);
    curY = drawTextBySentence(detail.xiang, LEFT, curY, W, 11, colors.text, 16);

    const yaoNames = ['初爻', '二爻', '三爻', '四爻', '五爻', '上爻'];
    for (let i = 0; i < detail.yaoCi.length; i++) {
      curY = drawText(yaoNames[i] + '：' + detail.yaoCi[i], LEFT, curY, W, 10, colors.gray, 15);
    }

    return curY;
  };

  y = drawGuaSection(guaDetail, '【本卦 ' + guaDetail.guaName + '】', y, hasBian && bianGuaDetail);

  if (hasBian && bianGuaDetail) {
    y += 18;
    drawLine(y);
    y += 18;
    y = drawGuaSection(bianGuaDetail, '【变卦 ' + bianGuaDetail.guaName + '】', y, true);
  }

  // 返回 Promise
  return new Promise((resolve, reject) => {
    ctx.draw(false, () => {
      wx.canvasToTempFilePath({
        canvasId: 'shareCanvas',
        success: (res) => {
          resolve(res.tempFilePath);
        },
        fail: (err) => {
          reject(err);
        }
      });
    });
  });
};

module.exports = {
  generateShareImage
};
