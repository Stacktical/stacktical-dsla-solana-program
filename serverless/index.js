import got from "got";
import functions from "@google-cloud/functions-framework";
import moment from "moment";
const ENDPOINT = "https://status.solana.com/api/v2/incidents.json";

functions.http("monthlyUptime", async (req, res) => {
  const json = await got.get(ENDPOINT).json();

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
      (startDate.month === lastMonth.month || endDate.month === lastMonth.month)
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

  res.send((monthlyDowntime / monthlyUptime) * 100);
  res.end();
});
