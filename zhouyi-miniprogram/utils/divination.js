/**
 * 核心算卦算法
 * 实现两种起卦方式：铜钱卦和大衍筮法
 */

// 随机生成 0 或 1
function random01() {
  return Math.random() < 0.5 ? 0 : 1;
}

/**
 * 铜钱卦 - 简单的随机起卦
 * 抛6次铜钱，每次结果：
 * - 2个正面以上为阳爻 (1)
 * - 2个正面以下为阴爻 (0)
 * - 3个正面为老阳 (变爻)
 * - 3个背面为老阴 (变爻)
 */
function coinDivinate() {
  const results = [];

  for (let i = 0; i < 6; i++) {
    // 抛3次铜钱，正面记1，背面记0
    let heads = 0;
    for (let j = 0; j < 3; j++) {
      heads += random01();
    }

    let yao;
    let type; // '少阳', '少阴', '老阳', '老阴'

    if (heads === 3) {
      yao = 1; // 老阳 - 阳爻变
      type = '老阳';
    } else if (heads === 2) {
      yao = 1; // 少阳 - 阳爻
      type = '少阳';
    } else if (heads === 1) {
      yao = 0; // 少阴 - 阴爻
      type = '少阴';
    } else {
      yao = 0; // 老阴 - 阴爻变
      type = '老阴';
    }

    results.push({ yao, type, detail: `第${i + 1}爻: ${type}` });
  }

  return results;
}

/**
 * 大衍筮法 - 传统周易算法
 * 使用蓍草进行复杂计算，得到老阴、少阳、少阴、老阳
 */
function dayanshiDivinate() {
  const results = [];

  for (let i = 0; i < 6; i++) {
    // 大衍之数五十，其用四十有九
    let total = 49;

    // 分而为二以象两
    let part1 = Math.floor(Math.random() * (total + 1));
    let part2 = total - part1;

    // 挂一以象三
    let selected = Math.random() < 0.5 ? 0 : 1;
    if (part1 === 1) selected = 1;
    if (part2 === 1) selected = 0;

    if (selected === 0) {
      part1 -= 1;
    } else {
      part2 -= 1;
    }

    // 揲四（数之以四以象四时）- 重复3次
    let yuSum = 0;

    for (let round = 0; round < 3; round++) {
      let yu1 = part1 % 4;
      let yu2 = part2 % 4;

      if (part1 === 0) {
        yu1 = 4;
        part2 -= 4;
      } else if (part2 === 0) {
        yu2 = 4;
        part1 -= 4;
      } else if (yu1 === 0) {
        yu1 = 4;
        yu2 = 4;
      }

      if (yu1 === 4 && yu2 === 4) {
        // 老阴阳
      } else {
        yuSum += (yu1 === 4 ? 0 : yu1) + (yu2 === 4 ? 0 : yu2);
      }

      part1 -= yu1;
      part2 -= yu2;
    }

    // 根据余数判断
    let type;
    let yao;

    if (yuSum === 0) {
      type = '老阴';
      yao = 0;
    } else if (yuSum === 1) {
      type = '少阳';
      yao = 1;
    } else if (yuSum === 2) {
      type = '少阴';
      yao = 0;
    } else if (yuSum === 3) {
      type = '老阳';
      yao = 1;
    } else {
      // 兜底处理
      type = '少阳';
      yao = 1;
    }

    results.push({ yao, type, detail: `第${i + 1}爻: ${type}` });
  }

  return results;
}

/**
 * 根据6爻结果计算卦象
 * @param {Array} yaoResults - 6个爻的结果数组
 * @returns {Object} - 卦象信息
 */
function calculateGua(yaoResults) {
  // 下卦（初爻到三爻）
  let lowerGua = yaoResults[0].yao * 4 + yaoResults[1].yao * 2 + yaoResults[2].yao;
  // 上卦（四爻到六爻）
  let upperGua = yaoResults[3].yao * 4 + yaoResults[4].yao * 2 + yaoResults[5].yao;

  return {
    lowerGua,
    upperGua,
    guaIndex: lowerGua * 8 + upperGua
  };
}

/**
 * 判断变爻，确定变卦
 * @param {Array} yaoResults - 6个爻的结果
 * @returns {Object} - 变卦信息
 */
function calculateBianGua(yaoResults) {
  let bianYaoIndices = [];

  yaoResults.forEach((result, index) => {
    if (result.type === '老阳' || result.type === '老阴') {
      bianYaoIndices.push(index);
    }
  });

  if (bianYaoIndices.length === 0) {
    return { hasBian: false, bianYaoIndices: [], bianGuaIndex: null };
  }

  // 计算变卦
  let bianYaoResults = yaoResults.map((result, index) => {
    if (result.type === '老阳') {
      return { ...result, yao: 0, type: '老阴（变）' };
    } else if (result.type === '老阴') {
      return { ...result, yao: 1, type: '老阳（变）' };
    }
    return result;
  });

  let lowerGua = bianYaoResults[0].yao * 4 + bianYaoResults[1].yao * 2 + bianYaoResults[2].yao;
  let upperGua = bianYaoResults[3].yao * 4 + bianYaoResults[4].yao * 2 + bianYaoResults[5].yao;

  return {
    hasBian: true,
    bianYaoIndices,
    bianGuaIndex: lowerGua * 8 + upperGua,
    bianYaoResults
  };
}

module.exports = {
  coinDivinate,
  dayanshiDivinate,
  calculateGua,
  calculateBianGua
};
