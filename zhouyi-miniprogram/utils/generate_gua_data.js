const fs = require('fs');
const path = require('path');

const jsonData = require('./ZhouyiFull.json');

// 八卦数据
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

// 转换为小程序需要的格式
const guaList = jsonData.map(g => {
  // 根据 binary 计算上下卦索引
  const upperBinary = g.upper_binary.join('');
  const lowerBinary = g.lower_binary.join('');
  const upperGuaIndex = parseInt(upperBinary, 2);
  const lowerGuaIndex = parseInt(lowerBinary, 2);

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

// 生成 gua_data.js 内容
const jsContent = `// 自动生成的文件 - 由 generate_gua_data.js 生成
// 此文件包含完整的64卦数据

const baGua = ${JSON.stringify(baGua, null, 2)};

const guaList = ${JSON.stringify(guaList, null, 2)};

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

  const upperIndex = parseInt(upperBinary, 2);
  const lowerIndex = parseInt(lowerBinary, 2);

  // 查找匹配的卦
  const gua = guaList.find(g =>
    g.upperGuaIndex === upperIndex && g.lowerGuaIndex === lowerIndex
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
