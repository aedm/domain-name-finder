export const Groups = ["18",
	"18-29",
	"30-39",
	"40-49",
	"50-59",
	"60-69",
	"70-79",
	"≥80"] as const;


export const RowLabels = ['population', 'hospital', 'death'] as const;

export type GroupType = typeof Groups[number];

export type RowType = typeof RowLabels[number];

export const VaccinationGroupsMapping: { [K: string]: GroupType } = {
	"12_15": "18",
	"16_17": "18",
	"18_24": "18-29",
	"25_29": "18-29",
	"30_34": "30-39",
	"35_39": "30-39",
	"40_44": "40-49",
	"45_49": "40-49",
	"50_54": "50-59",
	"55_59": "50-59",
	"60_64": "60-69",
	"65_69": "60-69",
	"70_74": "70-79",
	"75_79": "70-79",
	"80_84": "≥80",
	"85_89": "≥80",
	"90+": "≥80",
};

export type Row = {
	percentage: number,
	label: string,
	total: number,
}

export type Triplet = {
	name: string,
	rows: Row[],
};

export type SourceData = {
	[K in RowType]: {
		[T in GroupType]: number[];
	};
};