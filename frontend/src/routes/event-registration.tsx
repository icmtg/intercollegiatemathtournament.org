import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { useState, useEffect, type FormEvent } from "react";

export const Route = createFileRoute("/event-registration")({
  component: EventRegistration,
});

interface Event {
  id: string;
  name: string;
  description: string | null;
  location: string | null;
  startDate: string | null;
  endDate: string | null;
  registrationOpen: boolean;
}

function EventRegistration() {
  const navigate = useNavigate();
  const [events, setEvents] = useState<Event[]>([]);
  const [selectedEvent, setSelectedEvent] = useState<string>("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  // Form fields
  const [firstName, setFirstName] = useState("");
  const [lastName, setLastName] = useState("");
  const [email, setEmail] = useState("");
  const [tshirtSize, setTshirtSize] = useState("");
  const [division, setDivision] = useState("");
  const [expectedGraduationYear, setExpectedGraduationYear] = useState("");
  const [university, setUniversity] = useState("");
  const [resumeUrl, setResumeUrl] = useState("");
  const [acknowledgedIdRequirement, setAcknowledgedIdRequirement] = useState(false);
  const [acknowledgedFilming, setAcknowledgedFilming] = useState(false);
  const [acknowledgedTeamMerge, setAcknowledgedTeamMerge] = useState(false);
  const [interestedInFinancialAid, setInterestedInFinancialAid] = useState(false);

  useEffect(() => {
    // Fetch available events
    fetch("/api/events")
      .then((res) => res.json())
      .then((data) => {
        setEvents(data.events || []);
        if (data.events && data.events.length > 0) {
          setSelectedEvent(data.events[0].id);
        }
      })
      .catch((err) => {
        console.error("Failed to load events:", err);
        setError("Failed to load available events");
      });
  }, []);

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setError("");
    setLoading(true);

    if (!selectedEvent) {
      setError("Please select an event");
      setLoading(false);
      return;
    }

    try {
      const res = await fetch(`/api/events/${selectedEvent}/register`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({
          firstName,
          lastName,
          email,
          tshirtSize,
          division,
          expectedGraduationYear: parseInt(expectedGraduationYear),
          university,
          resumeUrl: resumeUrl || null,
          acknowledgedIdRequirement,
          acknowledgedFilming,
          acknowledgedTeamMerge,
          interestedInFinancialAid,
        }),
      });

      if (!res.ok) {
        const errorData = await res.json().catch(() => ({ error: "Registration failed" }));
        throw new Error(errorData.error || "Registration failed");
      }

      // Success - redirect to home or confirmation page
      navigate({ to: "/" });
    } catch (err) {
      setError(err instanceof Error ? err.message : "Registration failed");
    } finally {
      setLoading(false);
    }
  };

  const currentYear = new Date().getFullYear();
  const graduationYears = Array.from({ length: 10 }, (_, i) => currentYear + i);

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-3xl mx-auto">
        <div className="bg-white shadow-md rounded-lg p-8">
          <h2 className="text-3xl font-bold text-gray-900 mb-6">
            Event Registration
          </h2>

          {error && (
            <div className="mb-6 rounded-md bg-red-50 p-4">
              <p className="text-sm text-red-800">{error}</p>
            </div>
          )}

          <form onSubmit={handleSubmit} className="space-y-6">
            {/* Event Selection */}
            <div>
              <label htmlFor="event" className="block text-sm font-medium text-gray-700 mb-2">
                Select Event
              </label>
              <select
                id="event"
                required
                value={selectedEvent}
                onChange={(e) => setSelectedEvent(e.target.value)}
                className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
              >
                <option value="">-- Select an event --</option>
                {events.map((event) => (
                  <option key={event.id} value={event.id}>
                    {event.name}
                    {event.location && ` - ${event.location}`}
                  </option>
                ))}
              </select>
            </div>

            {/* Personal Information */}
            <div className="grid grid-cols-1 gap-6 sm:grid-cols-2">
              <div>
                <label htmlFor="firstName" className="block text-sm font-medium text-gray-700 mb-2">
                  First Name
                </label>
                <input
                  id="firstName"
                  type="text"
                  required
                  value={firstName}
                  onChange={(e) => setFirstName(e.target.value)}
                  className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                />
              </div>

              <div>
                <label htmlFor="lastName" className="block text-sm font-medium text-gray-700 mb-2">
                  Last Name
                </label>
                <input
                  id="lastName"
                  type="text"
                  required
                  value={lastName}
                  onChange={(e) => setLastName(e.target.value)}
                  className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                />
              </div>
            </div>

            <div>
              <label htmlFor="email" className="block text-sm font-medium text-gray-700 mb-2">
                Email Address
              </label>
              <input
                id="email"
                type="email"
                required
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
              />
            </div>

            <div>
              <label htmlFor="university" className="block text-sm font-medium text-gray-700 mb-2">
                University
              </label>
              <input
                id="university"
                type="text"
                required
                value={university}
                onChange={(e) => setUniversity(e.target.value)}
                className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
              />
            </div>

            <div className="grid grid-cols-1 gap-6 sm:grid-cols-3">
              <div>
                <label htmlFor="tshirtSize" className="block text-sm font-medium text-gray-700 mb-2">
                  T-Shirt Size
                </label>
                <select
                  id="tshirtSize"
                  required
                  value={tshirtSize}
                  onChange={(e) => setTshirtSize(e.target.value)}
                  className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                >
                  <option value="">Select size</option>
                  <option value="XS">XS</option>
                  <option value="S">S</option>
                  <option value="M">M</option>
                  <option value="L">L</option>
                  <option value="XL">XL</option>
                  <option value="XXL">XXL</option>
                </select>
              </div>

              <div>
                <label htmlFor="division" className="block text-sm font-medium text-gray-700 mb-2">
                  Division
                </label>
                <select
                  id="division"
                  required
                  value={division}
                  onChange={(e) => setDivision(e.target.value)}
                  className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                >
                  <option value="">Select division</option>
                  <option value="A">Division A</option>
                  <option value="B">Division B</option>
                </select>
              </div>

              <div>
                <label htmlFor="graduationYear" className="block text-sm font-medium text-gray-700 mb-2">
                  Expected Graduation Year
                </label>
                <select
                  id="graduationYear"
                  required
                  value={expectedGraduationYear}
                  onChange={(e) => setExpectedGraduationYear(e.target.value)}
                  className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                >
                  <option value="">Select year</option>
                  {graduationYears.map((year) => (
                    <option key={year} value={year}>
                      {year}
                    </option>
                  ))}
                </select>
              </div>
            </div>

            <div>
              <label htmlFor="resumeUrl" className="block text-sm font-medium text-gray-700 mb-2">
                Resume URL (Optional)
              </label>
              <input
                id="resumeUrl"
                type="url"
                value={resumeUrl}
                onChange={(e) => setResumeUrl(e.target.value)}
                placeholder="https://example.com/your-resume.pdf"
                className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
              />
            </div>

            {/* Acknowledgements */}
            <div className="space-y-4 border-t border-gray-200 pt-6">
              <h3 className="text-lg font-medium text-gray-900">Important Information</h3>

              <div className="flex items-start">
                <input
                  id="idRequirement"
                  type="checkbox"
                  required
                  checked={acknowledgedIdRequirement}
                  onChange={(e) => setAcknowledgedIdRequirement(e.target.checked)}
                  className="mt-1 h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                />
                <label htmlFor="idRequirement" className="ml-3 text-sm text-gray-700">
                  <strong>Required:</strong> Columbia's current security status only allows registered guests.
                  Please make sure to bring a government photo ID. ICMT is required to adhere to Columbia University's event/campus policies.
                </label>
              </div>

              <div className="flex items-start">
                <input
                  id="filming"
                  type="checkbox"
                  required
                  checked={acknowledgedFilming}
                  onChange={(e) => setAcknowledgedFilming(e.target.checked)}
                  className="mt-1 h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                />
                <label htmlFor="filming" className="ml-3 text-sm text-gray-700">
                  I acknowledge that Numberphile will likely be filming at this event.
                </label>
              </div>

              <div className="flex items-start">
                <input
                  id="teamMerge"
                  type="checkbox"
                  required
                  checked={acknowledgedTeamMerge}
                  onChange={(e) => setAcknowledgedTeamMerge(e.target.checked)}
                  className="mt-1 h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                />
                <label htmlFor="teamMerge" className="ml-3 text-sm text-gray-700">
                  I understand that if I am not registering as a full team, teams may be merged.
                </label>
              </div>

              <div className="flex items-start">
                <input
                  id="financialAid"
                  type="checkbox"
                  checked={interestedInFinancialAid}
                  onChange={(e) => setInterestedInFinancialAid(e.target.checked)}
                  className="mt-1 h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                />
                <label htmlFor="financialAid" className="ml-3 text-sm text-gray-700">
                  I am interested in learning more about financial aid (very limited availability).
                </label>
              </div>
            </div>

            {/* Submit Button */}
            <div className="pt-6">
              <button
                type="submit"
                disabled={loading}
                className="w-full flex justify-center py-3 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {loading ? "Submitting..." : "Complete Registration"}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}
