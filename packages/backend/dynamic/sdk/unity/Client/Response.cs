using System.Collections.Generic;
using Newtonsoft.Json;

namespace Rivet
{
    public class Response
    {
        public long StatusCode { get; private set; }
        public Dictionary<string, string> Headers { get; private set; }
        public string BodyRaw { get; private set; }
        public Dictionary<string, object> Body { get; private set; }

        public Response(long statusCode, Dictionary<string, string> headers, string bodyRaw)
        {
            this.StatusCode = statusCode;
            this.Headers = headers;
            this.BodyRaw = bodyRaw;

            try
            {
                this.Body = JsonConvert.DeserializeObject<Dictionary<string, object>>(bodyRaw);
            }
            catch (JsonException)
            {
                this.Body = null;
            }
        }

        public bool IsSuccess()
        {
            return StatusCode >= 200 && StatusCode < 300;
        }
    }
}
