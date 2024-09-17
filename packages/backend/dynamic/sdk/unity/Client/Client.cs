using System.Collections;
using UnityEngine;
using UnityEngine.Networking;
using Newtonsoft.Json;

namespace Rivet
{
    public class Client : MonoBehaviour
    {
        private Configuration configuration;

        public Client(Configuration configuration)
        {
            this.configuration = configuration;
        }

        public Request BuildRequest(string method, string path, Dictionary<string, object> body = null)
        {
            var url = this.BuildUrl(path);
            return new Request(this, method, url, body);
        }

        private string BuildUrl(string path)
        {
            return $"{this.configuration.Endpoint}/{path.TrimStart('/')}";
        }

        public IEnumerator SendRequest(Request request, System.Action<Response> callback)
        {
            UnityWebRequest unityRequest = new UnityWebRequest(request.Url, request.Method);
            if (request.Body != null)
            {
                string jsonData = JsonConvert.SerializeObject(request.Body);
                byte[] bodyRaw = System.Text.Encoding.UTF8.GetBytes(jsonData);
                unityRequest.uploadHandler = new UploadHandlerRaw(bodyRaw);
                unityRequest.downloadHandler = new DownloadHandlerBuffer();
                unityRequest.SetRequestHeader("Content-Type", "application/json");
            }

            yield return unityRequest.SendWebRequest();

            Response response = new Response(unityRequest.responseCode, unityRequest.GetResponseHeaders(), unityRequest.downloadHandler.text);
            callback?.Invoke(response);
        }
    }
}
