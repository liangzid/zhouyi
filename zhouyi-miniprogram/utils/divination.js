/**
 * 核心算卦算法
 * 实现两种起卦方式：铜钱卦和大衍筮法
 * 完全对应 Rust core.rs 中的实现
 */

/**
 * 铜钱卦 - 简单随机起卦
 * 与 Rust divination.rs 中的 coin_divinate 保持一致
 */
function coinDivinate(event) {
  const results = [];
  const typeLabels = [];

  for (let times = 0; times < 6; times++) {
    const x = Math.random() < 0.5 ? 0 : 1;
    results.push(x);
  }

  results.reverse();
  
  return results.map((yao, index) => ({
    yao,
    type: yao === 1 ? '少阳' : '少阴',
    detail: `第${index + 1}爻: ${yao === 1 ? '少阳' : '少阴'}`
  }));
}

/**
 * 大衍筮法 - 传统周易算法
 * 完全对应 Rust divination.rs 中的 dayanshi_divinate
 */
function dayanshiDivinate(event) {
  const results = [];
  const detailedList = [];
  const strList = [];

  for (let iYao = 0; iYao < 6; iYao++) {
    let shecao = 49;
    const thisDetailedList = [];

    for (let repetTimes = 0; repetTimes < 3; repetTimes++) {
      let split1 = Math.floor(Math.random() * (shecao + 1));
      let split2 = shecao - split1;

      if (repetTimes === 0) {
        let selectedIndex = Math.random() < 0.5 ? 0 : 1;
        if (split1 === 1 || split1 === 0) {
          selectedIndex = 1;
        } else if (split2 === 1 || split2 === 0) {
          selectedIndex = 0;
        }

        if (selectedIndex === 0) {
          split1 = split1 - 1;
        } else {
          split2 = split2 - 1;
        }
      }

      if (split1 === 0) {
        thisDetailedList.push([0, 4]);
        split2 -= 4;
      } else if (split2 === 0) {
        thisDetailedList.push([4, 0]);
        split1 -= 4;
      } else if (split1 % 4 === 0) {
        thisDetailedList.push([4, 4]);
        split1 -= 4;
        split2 -= 4;
      } else {
        const yu1 = split1 % 4;
        const yu2 = split2 % 4;
        thisDetailedList.push([yu1, yu2]);
        split1 -= yu1;
        split2 -= yu2;
      }

      shecao = split1 + split2;
    }

    const xangList = [];
    for (const element of thisDetailedList) {
      if (element[0] !== element[1]) {
        xangList.push(1);
      } else {
        xangList.push(0);
      }
    }

    thisDetailedList.reverse();
    detailedList.push(thisDetailedList);
    
    const res = xangList.reduce((a, b) => a + b, 0);
    let type;
    let yao;

    if (res === 0) {
      type = '老阴';
      yao = 0;
    } else if (res === 1) {
      type = '少阳';
      yao = 1;
    } else if (res === 2) {
      type = '少阴';
      yao = 0;
    } else if (res === 3) {
      type = '老阳';
      yao = 1;
    }

    strList.push(type);
    results.push({ yao, type, detail: `第${iYao + 1}爻: ${type}` });
  }

  results.reverse();
  detailedList.reverse();
  strList.reverse();

  return results;
}

// 八卦编号与 Rust 一致：0-乾,1-坤,2-震,3-艮,4-离,5-坎,6-兑,7-巽
const baGuaNames = ['乾', '坤', '震', '艮', '离', '坎', '兑', '巽'];

// 八卦二进制映射（从下到上）
const baGuaNum = {
  '111': 0,  // 乾
  '000': 1,  // 坤
  '001': 2,  // 震
  '100': 3,  // 艮
  '101': 4,  // 离
  '010': 5,  // 坎
  '011': 6,  // 兑
  '110': 7,  // 巽
};

/**
 * 根据6爻结果计算卦象
 * 与 Rust 逻辑保持一致
 * @param {Array} yaoResults - 6个爻的结果数组（从初爻到上爻的顺序）
 * @returns {Object} - 卦象信息
 */
function calculateGua(yaoResults) {
  const yaoValues = yaoResults.map(r => r.yao);
  
  const lowerBinary = String(yaoValues[0]) + String(yaoValues[1]) + String(yaoValues[2]);
  const upperBinary = String(yaoValues[3]) + String(yaoValues[4]) + String(yaoValues[5]);

  const lowerGua = baGuaNum[lowerBinary] || 0;
  const upperGua = baGuaNum[upperBinary] || 0;

  const guaName = baGuaNames[upperGua] + baGuaNames[lowerGua];
  
  const guaIndex = findGuaIndexByName(guaName);

  return {
    lowerGua,
    upperGua,
    guaIndex,
    guaName
  };
}

/**
 * 根据卦名找到对应的索引（按周易卦序）
 */
function findGuaIndexByName(guaName) {
  const guaOrder = [
    '乾乾', '坤坤', '坎震', '艮坎', '坎乾', '乾坎', '坤坎', '坎坤',
    '巽乾', '乾兑', '坤乾', '乾坤', '乾离', '离乾', '坤艮', '震坤',
    '兑震', '艮巽', '坤兑', '巽坤', '离震', '艮离', '艮坤', '坤震',
    '震乾', '艮乾', '艮震', '兑巽', '坎坎', '离离', '兑艮', '巽震',
    '艮乾', '震乾', '离地', '地离', '巽离', '离兑', '坎艮', '震坎',
    '艮兑', '巽震', '兑乾', '乾巽', '兑坤', '坤巽', '兑坎', '坎巽',
    '兑离', '离巽', '震震', '艮艮', '巽艮', '震兑', '震离', '离艮',
    '巽巽', '兑兑', '巽坎', '坎兑', '巽兑', '震艮', '坎离', '离坎'
  ];
  
  const index = guaOrder.indexOf(guaName);
  return index !== -1 ? index : 0;
}

/**
 * 判断变爻，确定变卦
 * @param {Array} yaoResults - 6个爻的结果
 * @returns {Object} - 变卦信息
 */
function calculateBianGua(yaoResults) {
  const bianYaoIndices = [];

  yaoResults.forEach((result, index) => {
    if (result.type === '老阳' || result.type === '老阴') {
      bianYaoIndices.push(index);
    }
  });

  if (bianYaoIndices.length === 0) {
    return { hasBian: false, bianYaoIndices: [], bianGuaIndex: null };
  }

  const bianYaoResults = yaoResults.map((result, index) => {
    if (result.type === '老阳') {
      return { ...result, yao: 0, type: '老阴（变）' };
    } else if (result.type === '老阴') {
      return { ...result, yao: 1, type: '老阳（变）' };
    }
    return result;
  });

  const bianGuaInfo = calculateGua(bianYaoResults);

  return {
    hasBian: true,
    bianYaoIndices,
    bianGuaIndex: bianGuaInfo.guaIndex,
    bianGuaName: bianGuaInfo.guaName,
    bianYaoResults
  };
}

function setQuestionSeed(questionInfo) {
}

module.exports = {
  coinDivinate,
  dayanshiDivinate,
  calculateGua,
  calculateBianGua,
  setQuestionSeed
};
