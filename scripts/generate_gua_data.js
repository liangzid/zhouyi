/**
 * 脚本：从 Structured_Zhouyi.json 生成 gua_data.js
 */

const fs = require('fs');
const path = require('path');

// 八卦数组索引: 0=乾, 1=坤, 2=震, 3=艮, 4=离, 5=坎, 6=兑, 7=巽
const idxToName = ["乾", "坤", "震", "艮", "离", "坎", "兑", "巽"];

// 读取 JSON 文件
const jsonPath = path.join(__dirname, '../core.rs/src/Structured_Zhouyi.json');
const zhouyiData = JSON.parse(fs.readFileSync(jsonPath, 'utf8'));

// 易经 64 卦完整映射表（按 guaIndex 顺序）- 必须与 JSON 中卦名完全一致
const guaMap = {
    "乾": {lower: 0, upper: 0},    // 0: 乾上乾下
    "坤": {lower: 1, upper: 1},    // 1: 坤上坤下
    "屯": {lower: 2, upper: 5},    // 2: 震下坎上
    "蒙": {lower: 5, upper: 3},    // 3: 坎下艮上
    "需": {lower: 0, upper: 5},    // 4: 乾下坎上
    "訟": {lower: 5, upper: 0},    // 5: 坎下乾上 (U+8AEF)
    "師": {lower: 5, upper: 1},    // 6: 坎下坤上
    "比": {lower: 1, upper: 5},    // 7: 坤下坎上
    "小畜": {lower: 0, upper: 7},  // 8: 乾下巽上
    "履": {lower: 6, upper: 0},    // 9: 兑下乾上
    "泰": {lower: 0, upper: 1},    // 10: 乾下坤上
    "否": {lower: 1, upper: 0},    // 11: 坤下乾上
    "同人": {lower: 0, upper: 4},  // 12: 乾下离上
    "大有": {lower: 4, upper: 0},  // 13: 离下乾上
    "謙": {lower: 3, upper: 1},    // 14: 艮下坤上
    "豫": {lower: 1, upper: 2},    // 15: 坤下震上
    "隨": {lower: 6, upper: 2},    // 16: 兑下震上
    "蠱": {lower: 4, upper: 3},    // 17: 离下艮上
    "臨": {lower: 1, upper: 2},    // 18: 坤下震上
    "觀": {lower: 4, upper: 7},    // 19: 离下巽上
    "噬嗑": {lower: 2, upper: 4},  // 20: 震下离上
    "賁": {lower: 4, upper: 5},    // 21: 离下坎上
    "剝": {lower: 3, upper: 1},    // 22: 艮下坤上
    "復": {lower: 2, upper: 1},    // 23: 震下坤上
    "無妄": {lower: 0, upper: 3},   // 24: 乾下艮上
    "大畜": {lower: 3, upper: 0},   // 25: 艮下乾上
    "頤": {lower: 2, upper: 3},     // 26: 震下艮上
    "大過": {lower: 6, upper: 4},   // 27: 兑下巽上
    "坎": {lower: 5, upper: 5},     // 28: 坎下坎上
    "離": {lower: 4, upper: 4},     // 29: 离下坎上
    "咸": {lower: 3, upper: 6},     // 30: 艮下兑上
    "恆": {lower: 2, upper: 7},     // 31: 震下巽上
    "遯": {lower: 3, upper: 0},     // 32: 艮下乾上
    "大壯": {lower: 0, upper: 2},   // 33: 乾下震上
    "晉": {lower: 1, upper: 4},     // 34: 坤下离上
    "明夷": {lower: 3, upper: 4},    // 35: 艮下离上
    "家人": {lower: 4, upper: 4},    // 36: 离下离上
    "睽": {lower: 4, upper: 6},     // 37: 离下兑上
    "蹇": {lower: 5, upper: 3},     // 38: 坎下艮上
    "解": {lower: 5, upper: 2},     // 39: 坎下震上
    "損": {lower: 3, upper: 6},     // 40: 艮下兑上
    "益": {lower: 7, upper: 2},     // 41: 巽下震上
    "夬": {lower: 0, upper: 6},      // 42: 乾下兑上
    "姤": {lower: 4, upper: 0},     // 43: 离下乾上
    "萃": {lower: 7, upper: 1},     // 44: 巽下坤上
    "升": {lower: 7, upper: 1},     // 45: 巽下坤上
    "困": {lower: 5, upper: 6},      // 46: 坎下兑上
    "井": {lower: 7, upper: 5},      // 47: 巽下坎上
    "革": {lower: 6, upper: 4},     // 48: 兑下离上
    "鼎": {lower: 6, upper: 4},     // 49: 兑下离上
    "震": {lower: 2, upper: 2},     // 50: 震下兑上
    "艮": {lower: 3, upper: 3},     // 51: 艮下兑上
    "漸": {lower: 4, upper: 7},      // 52: 离下巽上
    "歸妹": {lower: 5, upper: 6},    // 53: 坎下兑上
    "丰": {lower: 2, upper: 4},      // 54: 震下离上
    "旅": {lower: 7, upper: 4},      // 55: 巽下离上
    "巽": {lower: 7, upper: 7},     // 56: 乾下巽上
    "兌": {lower: 6, upper: 6},     // 57: 坤下巽上
    "渙": {lower: 7, upper: 2},      // 58: 巽下震上
    "節": {lower: 5, upper: 7},      // 59: 坎下巽上
    "中孚": {lower: 7, upper: 4},    // 60: 巽下离上
    "小過": {lower: 5, upper: 7},    // 61: 坎下巽上
    "既濟": {lower: 6, upper: 7},    // 62: 兑下巽上
    "未濟": {lower: 7, upper: 4},    // 63: 巽下离上
};

// 生成 64 卦数据
const guaData = [];

for (let guaIndex = 0; guaIndex < 64; guaIndex++) {
    const guaName = zhouyiData[guaIndex][0];
    const mapping = guaMap[guaName];

    if (!mapping) {
        console.log(`警告：未找到 ${guaName} (index ${guaIndex})`);
        guaData.push({
            guaIndex: guaIndex,
            guaName: guaName,
            guaCi: zhouyiData[guaIndex][1],
            lowerGua: 0,
            upperGua: 0,
            guaXiang: "未知",
            duan: zhouyiData[guaIndex][2],
            xiang: zhouyiData[guaIndex][3],
            yaoCi: zhouyiData[guaIndex][4],
            xiangCi: zhouyiData[guaIndex][5]
        });
    } else {
        guaData.push({
            guaIndex: guaIndex,
            guaName: guaName,
            guaCi: zhouyiData[guaIndex][1],
            lowerGua: mapping.lower,
            upperGua: mapping.upper,
            guaXiang: idxToName[mapping.lower] + idxToName[mapping.upper],
            duan: zhouyiData[guaIndex][2],
            xiang: zhouyiData[guaIndex][3],
            yaoCi: zhouyiData[guaIndex][4],
            xiangCi: zhouyiData[guaIndex][5]
        });
    }
}

// 生成 JS 文件
const baGuaExport = JSON.stringify(idxToName.map((name, i) => ({name, icon: "", num: i})), null, 4);
const guaDataStr = JSON.stringify(guaData, null, 8);

const jsContent = `/**
 * 64卦完整数据
 * 数据来源：Rust core.rs Structured_Zhouyi.json
 * 64卦生成规则：按易经标准顺序排列
 * 八卦序号：乾0 坤1 震2 艮3 离4 坎5 兑6 巽7
 */

const baGua = ${baGuaExport};

/**
 * 64卦数据数组
 * 按 guaIndex 顺序生成
 */
const guaData = ${guaDataStr};

/**
 * 根据卦象索引获取完整卦象信息
 */
function getGuaByIndex(guaIndex) {
  if (guaIndex < 0 || guaIndex >= guaData.length) {
    return null;
  }
  return guaData[guaIndex];
}

/**
 * 获取八卦信息
 */
function getBaGuaByIndex(index) {
  if (index < 0 || index >= baGua.length) {
    return null;
  }
  return baGua[index];
}

module.exports = {
  baGua,
  guaData,
  getGuaByIndex,
  getBaGuaByIndex
};
`;

// 写入文件
const outputPath = path.join(__dirname, '../zhouyi-miniprogram/utils/gua_data.js');
fs.writeFileSync(outputPath, jsContent);

console.log('生成完成！');
console.log('检查几个关键卦象：');
console.log('  guaIndex 0:', guaData[0].guaName, guaData[0].guaXiang);
console.log('  guaIndex 1:', guaData[1].guaName, guaData[1].guaXiang);
console.log('  guaIndex 2:', guaData[2].guaName, guaData[2].guaXiang);
console.log('  guaIndex 7:', guaData[7].guaName, guaData[7].guaXiang);
console.log('  guaIndex 8:', guaData[8].guaName, guaData[8].guaXiang);
console.log('  guaIndex 10:', guaData[10].guaName, guaData[10].guaXiang);
console.log('  guaIndex 42:', guaData[42].guaName, guaData[42].guaXiang);
console.log('  guaIndex 63:', guaData[63].guaName, guaData[63].guaXiang);
