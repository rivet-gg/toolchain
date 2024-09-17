using System.Collections.Generic;

namespace Rivet
{
    public class Request
    {
        public string Method { get; private set; }
        public string Url { get; private set; }
        public Dictionary<string, object> Body { get; private set; }

        private Client _client;

        public Request(Client client, string method, string url, Dictionary<string, object> body = null)
        {
            this._client = client;
            this.Method = method;
            this.Url = url;
            this.Body = body;
        }

        public void Send(System.Action<Response> callback)
        {
            _client.StartCoroutine(_client.SendRequest(this, callback));
        }
    }
}
