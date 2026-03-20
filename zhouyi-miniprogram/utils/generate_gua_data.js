const fs = require('fs');
const path = require('path');

const jsonData = require('./ZhouyiFull.json');

// 八卦数据 (JS order: 0=坤, 1=震, 2=坎, 3=兑, 4=艮, 5=离, 6=巽, 7=乾)
const baGua = [
  { name: '坤', xiang: '地', binary: [0, 0, 0] },
  { name: '震', xiang: '雷', binary: [0, 0, 1] },
  { name: '坎', xiang: '水', binary: [0, 1, 0] },
  { name: '兑', xiang: '泽', binary: [0, 1, 1] },
  { name: '艮', xiang: '山', binary: [1, 0, 0] },
  { name: '离', xiang: '火', binary: [1, 0, 1] },
  { name: '巽', xiang: '风', binary: [1, 1, 0] },
  { name: '乾', xiang: '天', binary: [1, 1, 1] }
];

// JS baGua index to Rust SubGua index mapping
// JS order: 0=坤, 1=震, 2=坎, 3=兑, 4=艮, 5=离, 6=巽, 7=乾
// Rust order: 0=乾, 1=坤, 2=震, 3=艮, 4=离, 5=坎, 6=兑, 7=巽
// Binary representation stays the same, but the index mapping differs
// JS index:  0   1   2   3   4   5   6   7
// Rust index:1   2   5   6   3   4   7   0
const jsToRustIndex = [1, 2, 5, 6, 3, 4, 7, 0];

// 转换 JS baGua index 为 Rust SubGua index
function toRustIndex(jsIndex) {
  return jsToRustIndex[jsIndex];
}

// 转换小程序需要的格式
const guaList = jsonData.map(g => {
  // 根据 binary 计算上下卦索引 (JS index)
  const upperBinary = g.upper_binary.join('');
  const lowerBinary = g.lower_binary.join('');
  const upperJsIndex = parseInt(upperBinary, 2);
  const lowerJsIndex = parseInt(lowerBinary, 2);

  // 转换为 Rust SubGua index
  const upperGuaIndex = toRustIndex(upperJsIndex);
  const lowerGuaIndex = toRustIndex(lowerJsIndex);

  return {
    guaIndex: g.gua_index,
    guaName: g.gua_name,
    guaCi: g.gua_ci,
    duan: g.duan,
    xiang: g.xiang,
    yaoCi: g.yao_ci,
    yaoXiang: g.yao_xiang,
    binaryVec: g.binary_vec,
    upperBinary: g.upper_binary,
    lowerBinary: g.lower_binary,
    upperGua: g.upper_gua,
    lowerGua: g.lower_gua,
    upperXiang: g.upper_xiang,
    lowerXiang: g.lower_xiang,
    guaXiang: g.gua_xiang,
    upperGuaIndex,
    lowerGuaIndex
  };
});

// JS baGua index to Rust SubGua index mapping (used for lookup)
const jsToRustIndexForLookup = [1, 2, 5, 6, 3, 4, 7, 0];

// 生成 gua_data.js 内容
const jsContent = `// 自动生成的文件 - 由 generate_gua_data.js 生成
// 此文件包含完整的64卦数据

const baGua = ${JSON.stringify(baGua, null, 2)};

const guaList = ${JSON.stringify(guaList, null, 2)};

// JS baGua index to Rust SubGua index mapping
// JS order: 0=坤, 1=震, 2=坎, 3=兑, 4=艮, 5=离, 6=巽, 7=乾
// Rust order: 0=乾, 1=坤, 2=震, 3=艮, 4=离, 5=坎, 6=兑, 7=巽
const jsToRustIndex = [1, 2, 5, 6, 3, 4, 7, 0];

function toRustIndex(jsIndex) {
  return jsToRustIndex[jsIndex];
}

// 通过卦名获取卦象数据
function getGuaByName(name) {
  return guaList.find(g => g.guaName === name);
}

// 通过索引获取卦象数据
function getGuaByIndex(index) {
  return guaList[index];
}

// 通过六爻结果计算卦象索引
function getGuaIndexByYaoResults(yaoResults) {
  // yaoResults: [初爻, 二爻, 三爻, 四爻, 五爻, 上爻] - 1为阳，0为阴
  // 二进制转换：从上到下：上爻(5)、五爻(4)、四爻(3)、三爻(2)、二爻(1)、初爻(0)
  // 上卦：四爻、五爻、上爻
  // 下卦：初爻、二爻、三爻

  const upper = [yaoResults[3], yaoResults[4], yaoResults[5]];
  const lower = [yaoResults[0], yaoResults[1], yaoResults[2]];

  const upperBinary = upper.join('');
  const lowerBinary = lower.join('');

  // JS binary index
  const upperJsIndex = parseInt(upperBinary, 2);
  const lowerJsIndex = parseInt(lowerBinary, 2);

  // Convert to Rust index for comparison with stored data
  const upperRustIndex = toRustIndex(upperJsIndex);
  const lowerRustIndex = toRustIndex(lowerJsIndex);

  // 查找匹配的卦
  const gua = guaList.find(g =>
    g.upperGuaIndex === upperRustIndex && g.lowerGuaIndex === lowerRustIndex
  );

  return gua ? gua.guaIndex : -1;
}

module.exports = {
  baGua,
  guaList,
  getGuaByName,
  getGuaByIndex,
  getGuaIndexByYaoResults
};
`;

fs.writeFileSync(path.join(__dirname, 'gua_data.js'), jsContent, 'utf-8');
console.log('生成 gua_data.js 成功！');
console.log('卦象数量:', guaList.length);
