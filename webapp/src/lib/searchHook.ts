import axios, {AxiosResponse} from 'axios';
import {asType} from '@/lib/asType';
import {ref, Ref} from 'vue';

export interface SearchInput {
    words: string;
    prefixes: string;
    postfixes: string;
    minWordCount: number;
    maxWordCount: number;
}

export interface SearchResult {
    free: string[];
    reserved: string[];
}

interface Payload {
    words: string[];
    prefixes: string[];
    postfixes: string[];
    minWordCount: number;
    maxWordCount: number;
}

interface SearchHook {
    setInput: (input: SearchInput) => void;
    result: Ref<SearchResult>;
}

export function useSearch(): SearchHook {
    const result = ref(asType<SearchResult>({free: [], reserved: []}));

    let payload = null as Payload | null;

    function setInput(input: SearchInput) {
        payload = makePayload(input);
        search();
    }

    let axiosPromise = null as Promise<AxiosResponse> | null;

    async function search() {
        if (!!axiosPromise) {
            return;
        }

        const payloadAsString = JSON.stringify(payload);
        axiosPromise = axios.post('/api/search', payload);
        const axiosResult = await axiosPromise;
        axiosPromise = null;

        result.value = asType<SearchResult>({
            free: sortList(axiosResult.data.free),
            reserved: sortList(axiosResult.data.reserved),
        });

        if (payloadAsString !== JSON.stringify(payload)) {
            setTimeout(search, 0);
        }
    }

    return {
        result,
        setInput,
    }
}

function sortList(list: string[]): string[] {
    let sorted = list ? [...list] : [];
    sorted.sort();
    return sorted;
}

function splitWords(s: string): string[] {
    return s.toLowerCase().split(/[\s,]+/).filter((x) => x != '');
}

function makePayload(input: SearchInput): Payload {
    const words = splitWords(input.words);
    const prefixes = splitWords(input.prefixes);
    const postfixes = splitWords(input.postfixes);
    return {
        maxWordCount: input.maxWordCount,
        minWordCount: input.minWordCount,
        words,
        prefixes,
        postfixes
    };
}