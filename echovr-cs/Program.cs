using System;
using System.Collections.Generic;
using System.Net.Http;
using System.Threading.Tasks;

namespace EchoVR
{
    class Program
    {
        private static async Task<Dictionary<String, object>> GetFrame(string address)
        {
            string baseAddress = $"http://{address}:6721";
            Console.WriteLine($"{baseAddress}/session");

            HttpClient client = new HttpClient();
            client.BaseAddress = new Uri(baseAddress);

            HttpResponseMessage resp;
            try {
                resp = await client.GetAsync("/session");
            } catch(Exception e) {
                Console.WriteLine($"HTTP error: {e.Message}");
                return null;
            }

            Dictionary<string, object> response;
            try {
                response = await resp.Content.ReadAsAsync<Dictionary<String, object>>();
            } catch(Exception e) {
                Console.WriteLine($"Response could not be decoded as JSON:\n{e.Message}");
                return null;
            }

            return response;
        }

        static async Task Main(string[] args)
        {
            string address = "127.0.0.1";
            if(args.Length > 0) {
                address = args[0];
            }

            while(true) {
                var response = await GetFrame(address);
                if(response == null) {
                    Console.WriteLine("Could not access API, trying again in 3 seconds");
                    await Task.Delay(3000);
                    continue;
                }

                Console.WriteLine($"{response}");
            }
        }
    }
}
