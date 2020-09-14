import axios from 'axios';
import { AxiosResponse } from 'axios';

async function getFrame(address: string) {
  const url = `http://${address}:6721/session`;

  let resp: AxiosResponse<any> | undefined = undefined;
  try {
    resp = await axios.get(url);
  } catch (e) {
    console.log(`HTTP error: ${e}`);
    return undefined;
  }

  return resp ? resp.data : undefined;
}

async function main() {
  const args = process.argv.slice(2);

  let address = '127.0.0.1';
  if (args.length > 0) {
    address = args[0];
  }

  while (true) {
    const response = await getFrame(address);
    if (!response) {
      console.log('Could not access API, trying again in 3 seconds');
      await new Promise(res => setTimeout(res, 3000));
      continue;
    }

    console.log(response);
  }
}

main();
