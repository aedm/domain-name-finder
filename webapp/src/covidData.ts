import { SourceData } from "./types";

export const CovidData: SourceData = {
	"population": {
		"18": [
			357480,
			21567,
			298507,
			19466,
			17134,
			806
		],
		"18-29": [
			80338,
			8245,
			25567,
			713,
			9031,
			36782
		],
		"30-39": [
			118564,
			8793,
			26841,
			658,
			7322,
			74950
		],
		"40-49": [
			158906,
			8666,
			16020,
			269,
			4130,
			129821
		],
		"50-59": [
			117264,
			5952,
			6922,
			91,
			1933,
			102366
		],
		"60-69": [
			67392,
			3484,
			2646,
			33,
			878,
			60351
		],
		"70-79": [
			37675,
			2049,
			1006,
			19,
			273,
			34328
		],
		"≥80": [
			13218,
			912,
			514,
			6,
			198,
			11588
		]
	},
	"hospital": {
		"18": [
			631,
			13,
			591,
			13,
			12,
			2
		],
		"18-29": [
			381,
			7,
			259,
			6,
			29,
			80
		],
		"30-39": [
			805,
			7,
			501,
			10,
			47,
			240
		],
		"40-49": [
			1183,
			17,
			572,
			8,
			47,
			539
		],
		"50-59": [
			1465,
			18,
			565,
			1,
			48,
			833
		],
		"60-69": [
			1585,
			7,
			354,
			4,
			33,
			1187
		],
		"70-79": [
			2160,
			8,
			264,
			4,
			29,
			1855
		],
		"≥80": [
			1969,
			3,
			207,
			1,
			33,
			1725
		]
	},
	"death": {
		"18": [
			8,
			1,
			7,
			0,
			0,
			0
		],
		"18-29": [
			9,
			0,
			8,
			0,
			0,
			1
		],
		"30-39": [
			34,
			1,
			21,
			0,
			2,
			10
		],
		"40-49": [
			82,
			2,
			40,
			0,
			2,
			38
		],
		"50-59": [
			196,
			5,
			84,
			0,
			5,
			102
		],
		"60-69": [
			482,
			3,
			124,
			0,
			16,
			339
		],
		"70-79": [
			924,
			4,
			140,
			0,
			15,
			765
		],
		"≥80": [
			1695,
			5,
			163,
			0,
			50,
			1477
		]
	}
};

export const VaccinationsAgeDemographics =
	[
		{
			"age": "12_15",
			"VaccineRegisterPopulationByVaccinationDate": 2878154.0,
			"cumPeopleVaccinatedCompleteByVaccinationDate": 13472.0,
			"newPeopleVaccinatedCompleteByVaccinationDate": 258.0,
			"cumPeopleVaccinatedFirstDoseByVaccinationDate": 1077760.0,
			"newPeopleVaccinatedFirstDoseByVaccinationDate": 10921.0,
			"cumPeopleVaccinatedSecondDoseByVaccinationDate": 13472.0,
			"newPeopleVaccinatedSecondDoseByVaccinationDate": 258.0,
			"cumVaccinationFirstDoseUptakeByVaccinationDatePercentage": 37.4,
			"cumVaccinationCompleteCoverageByVaccinationDatePercentage": 0.5,
			"cumVaccinationSecondDoseUptakeByVaccinationDatePercentage": 0.5
		},
		{
			"age": "16_17",
			"VaccineRegisterPopulationByVaccinationDate": 1368283.0,
			"cumPeopleVaccinatedCompleteByVaccinationDate": 182224.0,
			"newPeopleVaccinatedCompleteByVaccinationDate": 2079.0,
			"cumPeopleVaccinatedFirstDoseByVaccinationDate": 796668.0,
			"newPeopleVaccinatedFirstDoseByVaccinationDate": 1246.0,
			"cumPeopleVaccinatedSecondDoseByVaccinationDate": 182224.0,
			"newPeopleVaccinatedSecondDoseByVaccinationDate": 2079.0,
			"cumVaccinationFirstDoseUptakeByVaccinationDatePercentage": 58.2,
			"cumVaccinationCompleteCoverageByVaccinationDatePercentage": 13.3,
			"cumVaccinationSecondDoseUptakeByVaccinationDatePercentage": 13.3
		},
		{
			"age": "18_24",
			"VaccineRegisterPopulationByVaccinationDate": 5319420.0,
			"cumPeopleVaccinatedCompleteByVaccinationDate": 3086199.0,
			"newPeopleVaccinatedCompleteByVaccinationDate": 4140.0,
			"cumPeopleVaccinatedFirstDoseByVaccinationDate": 3594617.0,
			"newPeopleVaccinatedFirstDoseByVaccinationDate": 2083.0,
			"cumPeopleVaccinatedSecondDoseByVaccinationDate": 3086199.0,
			"newPeopleVaccinatedSecondDoseByVaccinationDate": 4140.0,
			"cumVaccinationFirstDoseUptakeByVaccinationDatePercentage": 67.6,
			"cumVaccinationCompleteCoverageByVaccinationDatePercentage": 58.0,
			"cumVaccinationSecondDoseUptakeByVaccinationDatePercentage": 58.0
		},
		{
			"age": "25_29",
			"VaccineRegisterPopulationByVaccinationDate": 4498658.0,
			"cumPeopleVaccinatedCompleteByVaccinationDate": 2622123.0,
			"newPeopleVaccinatedCompleteByVaccinationDate": 2545.0,
			"cumPeopleVaccinatedFirstDoseByVaccinationDate": 2939825.0,
			"newPeopleVaccinatedFirstDoseByVaccinationDate": 1378.0,
			"cumPeopleVaccinatedSecondDoseByVaccinationDate": 2622123.0,
			"newPeopleVaccinatedSecondDoseByVaccinationDate": 2545.0,
			"cumVaccinationFirstDoseUptakeByVaccinationDatePercentage": 65.3,
			"cumVaccinationCompleteCoverageByVaccinationDatePercentage": 58.3,
			"cumVaccinationSecondDoseUptakeByVaccinationDatePercentage": 58.3
		},
		{
			"age": "30_34",
			"VaccineRegisterPopulationByVaccinationDate": 4811300.0,
			"cumPeopleVaccinatedCompleteByVaccinationDate": 2961209.0,
			"newPeopleVaccinatedCompleteByVaccinationDate": 2242.0,
			"cumPeopleVaccinatedFirstDoseByVaccinationDate": 3250585.0,
			"newPeopleVaccinatedFirstDoseByVaccinationDate": 1288.0,
			"cumPeopleVaccinatedSecondDoseByVaccinationDate": 2961209.0,
			"newPeopleVaccinatedSecondDoseByVaccinationDate": 2242.0,
			"cumVaccinationFirstDoseUptakeByVaccinationDatePercentage": 67.6,
			"cumVaccinationCompleteCoverageByVaccinationDatePercentage": 61.5,
			"cumVaccinationSecondDoseUptakeByVaccinationDatePercentage": 61.5
		},
		{
			"age": "35_39",
			"VaccineRegisterPopulationByVaccinationDate": 4574227.0,
			"cumPeopleVaccinatedCompleteByVaccinationDate": 3046048.0,
			"newPeopleVaccinatedCompleteByVaccinationDate": 1663.0,
			"cumPeopleVaccinatedFirstDoseByVaccinationDate": 3269586.0,
			"newPeopleVaccinatedFirstDoseByVaccinationDate": 919.0,
			"cumPeopleVaccinatedSecondDoseByVaccinationDate": 3046048.0,
			"newPeopleVaccinatedSecondDoseByVaccinationDate": 1663.0,
			"cumVaccinationFirstDoseUptakeByVaccinationDatePercentage": 71.5,
			"cumVaccinationCompleteCoverageByVaccinationDatePercentage": 66.6,
			"cumVaccinationSecondDoseUptakeByVaccinationDatePercentage": 66.6
		},
		{
			"age": "40_44",
			"VaccineRegisterPopulationByVaccinationDate": 4201161.0,
			"cumPeopleVaccinatedCompleteByVaccinationDate": 3055589.0,
			"newPeopleVaccinatedCompleteByVaccinationDate": 1142.0,
			"cumPeopleVaccinatedFirstDoseByVaccinationDate": 3220621.0,
			"newPeopleVaccinatedFirstDoseByVaccinationDate": 546.0,
			"cumPeopleVaccinatedSecondDoseByVaccinationDate": 3055589.0,
			"newPeopleVaccinatedSecondDoseByVaccinationDate": 1142.0,
			"cumVaccinationFirstDoseUptakeByVaccinationDatePercentage": 76.7,
			"cumVaccinationCompleteCoverageByVaccinationDatePercentage": 72.7,
			"cumVaccinationSecondDoseUptakeByVaccinationDatePercentage": 72.7
		},
		{
			"age": "45_49",
			"VaccineRegisterPopulationByVaccinationDate": 3989024.0,
			"cumPeopleVaccinatedCompleteByVaccinationDate": 3143806.0,
			"newPeopleVaccinatedCompleteByVaccinationDate": 761.0,
			"cumPeopleVaccinatedFirstDoseByVaccinationDate": 3268844.0,
			"newPeopleVaccinatedFirstDoseByVaccinationDate": 384.0,
			"cumPeopleVaccinatedSecondDoseByVaccinationDate": 3143806.0,
			"newPeopleVaccinatedSecondDoseByVaccinationDate": 761.0,
			"cumVaccinationFirstDoseUptakeByVaccinationDatePercentage": 81.9,
			"cumVaccinationCompleteCoverageByVaccinationDatePercentage": 78.8,
			"cumVaccinationSecondDoseUptakeByVaccinationDatePercentage": 78.8
		},
		{
			"age": "50_54",
			"VaccineRegisterPopulationByVaccinationDate": 4242684.0,
			"cumPeopleVaccinatedCompleteByVaccinationDate": 3566469.0,
			"newPeopleVaccinatedCompleteByVaccinationDate": 622.0,
			"cumPeopleVaccinatedFirstDoseByVaccinationDate": 3669708.0,
			"newPeopleVaccinatedFirstDoseByVaccinationDate": 289.0,
			"cumPeopleVaccinatedSecondDoseByVaccinationDate": 3566469.0,
			"newPeopleVaccinatedSecondDoseByVaccinationDate": 622.0,
			"cumVaccinationFirstDoseUptakeByVaccinationDatePercentage": 86.5,
			"cumVaccinationCompleteCoverageByVaccinationDatePercentage": 84.1,
			"cumVaccinationSecondDoseUptakeByVaccinationDatePercentage": 84.1
		},
		{
			"age": "55_59",
			"VaccineRegisterPopulationByVaccinationDate": 4117556.0,
			"cumPeopleVaccinatedCompleteByVaccinationDate": 3567319.0,
			"newPeopleVaccinatedCompleteByVaccinationDate": 635.0,
			"cumPeopleVaccinatedFirstDoseByVaccinationDate": 3660375.0,
			"newPeopleVaccinatedFirstDoseByVaccinationDate": 236.0,
			"cumPeopleVaccinatedSecondDoseByVaccinationDate": 3567319.0,
			"newPeopleVaccinatedSecondDoseByVaccinationDate": 635.0,
			"cumVaccinationFirstDoseUptakeByVaccinationDatePercentage": 88.9,
			"cumVaccinationCompleteCoverageByVaccinationDatePercentage": 86.6,
			"cumVaccinationSecondDoseUptakeByVaccinationDatePercentage": 86.6
		},
		{
			"age": "60_64",
			"VaccineRegisterPopulationByVaccinationDate": 3506140.0,
			"cumPeopleVaccinatedCompleteByVaccinationDate": 3107063.0,
			"newPeopleVaccinatedCompleteByVaccinationDate": 470.0,
			"cumPeopleVaccinatedFirstDoseByVaccinationDate": 3178449.0,
			"newPeopleVaccinatedFirstDoseByVaccinationDate": 186.0,
			"cumPeopleVaccinatedSecondDoseByVaccinationDate": 3107063.0,
			"newPeopleVaccinatedSecondDoseByVaccinationDate": 470.0,
			"cumVaccinationFirstDoseUptakeByVaccinationDatePercentage": 90.7,
			"cumVaccinationCompleteCoverageByVaccinationDatePercentage": 88.6,
			"cumVaccinationSecondDoseUptakeByVaccinationDatePercentage": 88.6
		},
		{
			"age": "65_69",
			"VaccineRegisterPopulationByVaccinationDate": 2918933.0,
			"cumPeopleVaccinatedCompleteByVaccinationDate": 2661082.0,
			"newPeopleVaccinatedCompleteByVaccinationDate": 252.0,
			"cumPeopleVaccinatedFirstDoseByVaccinationDate": 2697349.0,
			"newPeopleVaccinatedFirstDoseByVaccinationDate": 159.0,
			"cumPeopleVaccinatedSecondDoseByVaccinationDate": 2661082.0,
			"newPeopleVaccinatedSecondDoseByVaccinationDate": 252.0,
			"cumVaccinationFirstDoseUptakeByVaccinationDatePercentage": 92.4,
			"cumVaccinationCompleteCoverageByVaccinationDatePercentage": 91.2,
			"cumVaccinationSecondDoseUptakeByVaccinationDatePercentage": 91.2
		},
		{
			"age": "70_74",
			"VaccineRegisterPopulationByVaccinationDate": 2861729.0,
			"cumPeopleVaccinatedCompleteByVaccinationDate": 2680173.0,
			"newPeopleVaccinatedCompleteByVaccinationDate": 197.0,
			"cumPeopleVaccinatedFirstDoseByVaccinationDate": 2705051.0,
			"newPeopleVaccinatedFirstDoseByVaccinationDate": 129.0,
			"cumPeopleVaccinatedSecondDoseByVaccinationDate": 2680173.0,
			"newPeopleVaccinatedSecondDoseByVaccinationDate": 197.0,
			"cumVaccinationFirstDoseUptakeByVaccinationDatePercentage": 94.5,
			"cumVaccinationCompleteCoverageByVaccinationDatePercentage": 93.7,
			"cumVaccinationSecondDoseUptakeByVaccinationDatePercentage": 93.7
		},
		{
			"age": "75_79",
			"VaccineRegisterPopulationByVaccinationDate": 2157022.0,
			"cumPeopleVaccinatedCompleteByVaccinationDate": 2046261.0,
			"newPeopleVaccinatedCompleteByVaccinationDate": 102.0,
			"cumPeopleVaccinatedFirstDoseByVaccinationDate": 2062986.0,
			"newPeopleVaccinatedFirstDoseByVaccinationDate": 76.0,
			"cumPeopleVaccinatedSecondDoseByVaccinationDate": 2046261.0,
			"newPeopleVaccinatedSecondDoseByVaccinationDate": 102.0,
			"cumVaccinationFirstDoseUptakeByVaccinationDatePercentage": 95.6,
			"cumVaccinationCompleteCoverageByVaccinationDatePercentage": 94.9,
			"cumVaccinationSecondDoseUptakeByVaccinationDatePercentage": 94.9
		},
		{
			"age": "80_84",
			"VaccineRegisterPopulationByVaccinationDate": 1439688.0,
			"cumPeopleVaccinatedCompleteByVaccinationDate": 1366738.0,
			"newPeopleVaccinatedCompleteByVaccinationDate": 66.0,
			"cumPeopleVaccinatedFirstDoseByVaccinationDate": 1378620.0,
			"newPeopleVaccinatedFirstDoseByVaccinationDate": 44.0,
			"cumPeopleVaccinatedSecondDoseByVaccinationDate": 1366738.0,
			"newPeopleVaccinatedSecondDoseByVaccinationDate": 66.0,
			"cumVaccinationFirstDoseUptakeByVaccinationDatePercentage": 95.8,
			"cumVaccinationCompleteCoverageByVaccinationDatePercentage": 94.9,
			"cumVaccinationSecondDoseUptakeByVaccinationDatePercentage": 94.9
		},
		{
			"age": "85_89",
			"VaccineRegisterPopulationByVaccinationDate": 888819.0,
			"cumPeopleVaccinatedCompleteByVaccinationDate": 842836.0,
			"newPeopleVaccinatedCompleteByVaccinationDate": 59.0,
			"cumPeopleVaccinatedFirstDoseByVaccinationDate": 851255.0,
			"newPeopleVaccinatedFirstDoseByVaccinationDate": 31.0,
			"cumPeopleVaccinatedSecondDoseByVaccinationDate": 842836.0,
			"newPeopleVaccinatedSecondDoseByVaccinationDate": 59.0,
			"cumVaccinationFirstDoseUptakeByVaccinationDatePercentage": 95.8,
			"cumVaccinationCompleteCoverageByVaccinationDatePercentage": 94.8,
			"cumVaccinationSecondDoseUptakeByVaccinationDatePercentage": 94.8
		},
		{
			"age": "90+",
			"VaccineRegisterPopulationByVaccinationDate": 503446.0,
			"cumPeopleVaccinatedCompleteByVaccinationDate": 468630.0,
			"newPeopleVaccinatedCompleteByVaccinationDate": 43.0,
			"cumPeopleVaccinatedFirstDoseByVaccinationDate": 474536.0,
			"newPeopleVaccinatedFirstDoseByVaccinationDate": 6.0,
			"cumPeopleVaccinatedSecondDoseByVaccinationDate": 468630.0,
			"newPeopleVaccinatedSecondDoseByVaccinationDate": 43.0,
			"cumVaccinationFirstDoseUptakeByVaccinationDatePercentage": 94.3,
			"cumVaccinationCompleteCoverageByVaccinationDatePercentage": 93.1,
			"cumVaccinationSecondDoseUptakeByVaccinationDatePercentage": 93.1
		}
	];
