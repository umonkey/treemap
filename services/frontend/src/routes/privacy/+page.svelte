<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { locale } from '$lib/locale';
</script>

<Dialog title={locale.sidePrivacy()}>
	<article>
		<p>
			This Privacy Policy describes how we collect, use, and handle your personal information when
			you use the Trees of Yerevan application.
		</p>

		<h2>Information We Collect</h2>
		<ul>
			<li>
				<strong>Account Information:</strong> when you sign in using Google, we receive your email address,
				name, and profile picture. This information is used to identify your contributions to the tree
				map.
			</li>
			<li>
				<strong>Location Data:</strong> the application uses your device's location to show your position
				on the map and to allow you to accurately add trees to the database. We do not track your location
				in the background.
			</li>
			<li>
				<strong>Photos:</strong> if you choose to upload photos of trees, these photos are stored on
				our servers and made public as part of the tree map.
			</li>
			<li>
				<strong>Device Info:</strong> we may collect basic information about your device and application
				performance to help us fix bugs and improve the app.
			</li>
		</ul>

		<h2>How We Use Information</h2>
		<p>We use the collected information to:</p>
		<ul>
			<li>Provide and maintain the tree mapping service.</li>
			<li>Attribute tree data and photos to their respective contributors.</li>
			<li>Communicate with you regarding your contributions if necessary.</li>
			<li>Analyze and improve the application performance.</li>
		</ul>

		<h2>Data Storage and Security</h2>
		<p>
			Your data is stored securely on our servers. We implement appropriate technical and
			organizational measures to protect your personal information.
		</p>

		<h2>Your Rights</h2>
		<p>You have the right to:</p>
		<ul>
			<li>Access the personal information we hold about you.</li>
			<li>Request the deletion of your account and associated personal data.</li>
			<li>Object to the processing of your data.</li>
		</ul>

		<h2>Contact Us</h2>
		<p>
			If you have any questions about this Privacy Policy or our data practices, please contact us
			via GitHub issues or at the contact email provided in the Google Play Store.
		</p>
	</article>
</Dialog>

<style>
	article {
		padding: 1rem;
		line-height: 1.6;
	}

	h2 {
		font-size: 1.2rem;
		margin-top: 1.5rem;
		margin-bottom: 0.5rem;
	}

	ul {
		margin-bottom: 1rem;
	}

	li {
		margin-bottom: 0.5rem;
	}
</style>
