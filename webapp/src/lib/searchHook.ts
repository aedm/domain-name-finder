import axios, { AxiosResponse } from 'axios';
import { asType } from '@/lib/asType';
import { ref, Ref } from 'vue';

export interface SearchInput {
  words: string;
  prefixes: string;
  postfixes: string;
}

export interface SearchResult {
  free: string[];
  reserved: string[];
}

interface Payload {
  words: string[];
}

let axiosPromise = null as Promise<AxiosResponse> | null;
let lastPayloadAsString = '';
let lastResult = null as SearchResult | null;

interface SearchHook {
  setInput: (input: SearchInput) => void;
  result: Ref<SearchResult>;
}

export function useSearch(): SearchHook {
  let axiosPromise = null as Promise<AxiosResponse> | null;
  let lastPayloadAsString = '';
  let lastResult = null as SearchResult | null;

  const result = ref(asType<SearchResult>({ free: [], reserved: [] }));

  let currentInput = null as SearchInput | null;

  async function search() {

  }

  function setInput(input: SearchInput) {
    currentInput = input;
    search();
  }

  return {
    result,
    setInput,
  }
}

export async function search(input: SearchInput): Promise<SearchResult> {
  if (!!axiosPromise) {
    throw 'Multiple concurrent requests.';
  }
  const payload = makePayload(input);
  const payloadAsString = JSON.stringify(payload);
  if (payloadAsString === lastPayloadAsString) {
    return lastResult!;
  }
  console.log("AXIOS", payloadAsString);
  axiosPromise = axios.post('/api/search', payload);
  const axiosResult = await axiosPromise;
  axiosPromise = null;

  const result = asType<SearchResult>({
    free: sortList(axiosResult.data.free),
    reserved: sortList(axiosResult.data.reserved),
  });

  lastPayloadAsString = payloadAsString;
  lastResult = result;
  return result;
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
  const words = searchInput.toLowerCase().split(/[\s,]+/).filter((x) => x != '');
  return asType<Payload>({
    words,
  });
}