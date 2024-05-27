const axios = require("axios");

const NUM_REQUESTS = 10000; // Number of requests to send

async function sendRequests() {
  const startTime = Date.now(); // Start time for measuring duration
  const promises = [];

  for (let i = 0; i < NUM_REQUESTS; i++) {
    promises.push(sendRequest(i));
  }

  try {
    await Promise.all(promises);
    const duration = (Date.now() - startTime) / 1000; // Duration in seconds
    const avgRequestsPerSecond = NUM_REQUESTS / duration;
    console.log(
      `Average requests per second: ${avgRequestsPerSecond.toFixed(2)}`
    );
  } catch (error) {
    console.error("Error sending requests:", error.message);
  }
}

async function sendRequest(counter) {
  const requestBody = JSON.stringify({
    message: {
      token:
        "c38QVqNdTNeVGZiTwDnYDZ:APA91bEkrsDSqtTcWsc0CVQ1mTEWA-Y6S-iB7UhEu8jDaPRhVUOg_4Ep9SXvmnHcIZZ2nKp7VyfHKU7IbFpmCsLQsT01mrnsrgDL8a8mZkL7aRgVTLJKc5yekyEAnlTVA5Eeas1X-26m",
      notification: {
        title: "Title new",
        body: `This is a test message. Counter: ${counter}`,
      },
    },
  });

  try {
    const response = await axios.post(
      "https://fcm.googleapis.com/v1/projects/demonotification-23701/messages:send",
      requestBody,
      {
        headers: {
          "Content-Type": "application/json",
          Authorization:
            "Bearer ya29.c.c0AY_VpZhF88LVuhNlRZpzO7BvK9Zo8eKvuQ3AnisxX6EGBDcb33N4fCrKhVV-F-Lh9JeZJkQMAMMjIxbDsCwNKXaWEemD1hYRK8HhuG6DoWNgGl8AeYzceRYQnPgqSCnjtzpmr-Uhee9fq_P5vUTb4jMcmkmicIPOPWjcSLshLEvIMiD6mosbBkMFAkuSJvUGd9GPU3AlsiP_V_TRK8e8HWPHeHlKtqHjMnMPPw_coHaqYRpP4xYNlo-PVqHi0A9gTbri2ik4okRKk-c6nSFAevcDm2-dUn-QXnxko_9RcEpoyoARZo1MI3ecL8Ajxx_VhOYvw7qGJD-oIgwKuxC83g8aU_J8ixEFL5jQTdUDkDJ0B8ng2lUSfwgIT385C-o7zbIv4ZeeV1hgy61ihUkYgqY24qX-hy3lXFtOWveWIWhnUl7IpW1bi9Z83_eF6YvViqfjo0rlzQBz4biOkJMYIpbe2Yjeamc50hwXB41dJrSueOOsu3sb_hlX3gbjI_k-rgaWJYBlUz61_UFMrJb_cgywf7SjoOXuQsMVFvU_9r27kSp9pp4xz-ol4_M1tuzX-z7WyYjmjrZYzvk_o0luW3SI3wfWFMYlvQgl3fx6OJyyu5VQSh8XBeXVRf13wMgRXlmzl-euyJ7FpW2-uyhi1bZqkBhSpzqpxe24Ow1y7J4j1qmMScxk3Sr3_t14ebVqq6YquzRbuVarz4xQgxxjtuayWw_OxVzBwz3JM9Ml0nyYR9RUaVry8YcBbYqUccelMeFjufZ11fMnc6tvuu6yFt5jhUk9eoeZv962FdXjUSJdod27743VeqZ1e9SWvq2WWxnc5kuRq5quwZ41XulniaUW5VcnImZS3Fc6ofpjOet_XrfV75yF_vmWZ6Vg0wnxBcadFaiQBWgf-YZJq8Wrusygrfv32kRdljxzSRhx0J6jRnf8QrumVrmZwt4hR7IesWXmF6VWZZwF80xcfX3pyhMsQUZnhYFOMcSd3MF8uwlppccfxmBYu1X",
        },
      }
    );

    if (response.status === 200) {
      console.log(`Notification sent successfully. Counter: ${counter}`);
    } else {
      console.log(
        `Failed to send notification. Status code: ${response.status}. Counter: ${counter}`
      );
    }
  } catch (error) {
    console.error("Error sending request:", error.message);
  }
}

sendRequests();
