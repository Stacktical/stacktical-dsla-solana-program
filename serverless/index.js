import got from "got";
import moment from "moment";
const functions = require("@google-cloud/functions-framework");

const ENDPOINT = "https://status.solana.com/api/v2/incidents.json";

functions.http("monthlyUptime", (req, res) => {
  got
    .get(ENDPOINT)
    .json()
    .then((json) => {
      const incidents = json.incidents;
      const lastMonth = moment().subtract(1, "month");

      let monthlyUptime =
        lastMonth.endOf("month").unix() - lastMonth.startOf("month").unix();
      let monthlyDowntime = 0;

      for (let incident of incidents) {
        let startDate = moment(incident.started_at);
        let endDate = moment(incident.resolved_at);
        if (
          incident.impact === "major" &&
          (startDate.month === lastMonth.month ||
            endDate.month === lastMonth.month)
        ) {
          if (startDate.isBefore(lastMonth.startOf("month"))) {
            startDate = lastMonth.startOf("month");
          }
          if (endDate.isAfter(lastMonth.endOf("month"))) {
            endDate = lastMonth.endOf("month");
          }
          monthlyDowntime = monthlyUptime + (endDate.unix() - startDate.unix());
          monthlyUptime = monthlyUptime - monthlyDowntime;
        }
      }

      res.status(200).send((monthlyDowntime / monthlyUptime) * 100);
      res.end();
    });
});
