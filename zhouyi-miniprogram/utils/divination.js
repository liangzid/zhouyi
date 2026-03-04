/**
 * 核心算卦算法
 * 实现两种起卦方式：铜钱卦和大衍筮法
 * 使用 gua_data.js 中的正确八卦映射
 */

const guaData = require('./gua_data.js');

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

// 八卦编号与 gua_data.js 一致：0-坤,1-震,2-坎,3-兑,4-艮,5-离,6-巽,7-乾
const baGuaNames = ['坤', '震', '坎', '兑', '艮', '离', '巽', '乾'];

// 八卦二进制映射（从下到上）- 与 gua_data.js 一致
const baGuaNum = {
  '000': 0,  // 坤
  '001': 1,  // 震
  '010': 2,  // 坎
  '011': 3,  // 兑
  '100': 4,  // 艮
  '101': 5,  // 离
  '110': 6,  // 巽
  '111': 7,  // 乾
};

/**
 * 根据6爻结果计算卦象
 * 使用 gua_data.js 中的正确映射
 * @param {Array} yaoResults - 6个爻的结果数组（从初爻到上爻的顺序）
 * @returns {Object} - 卦象信息
 */
function calculateGua(yaoResults) {
  const yaoValues = yaoResults.map(r => r.yao);

  // 下卦：初爻、二爻、三爻（索引 0, 1, 2）
  const lowerBinary = String(yaoValues[0]) + String(yaoValues[1]) + String(yaoValues[2]);
  // 上卦：四爻、五爻、上爻（索引 3, 4, 5）
  const upperBinary = String(yaoValues[3]) + String(yaoValues[4]) + String(yaoValues[5]);

  const lowerGua = baGuaNum[lowerBinary] || 0;
  const upperGua = baGuaNum[upperBinary] || 0;

  // 使用 gua_data.js 中的方法获取卦象索引
  const guaIndex = guaData.getGuaIndexByYaoResults(yaoValues);
  const guaInfo = guaData.getGuaByIndex(guaIndex);

  return {
    lowerGua,
    upperGua,
    guaIndex,
    guaName: guaInfo ? guaInfo.guaName : ''
  };
}

// 保留此函数以兼容旧代码
function findGuaIndexByName(guaName) {
  return 0;
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
