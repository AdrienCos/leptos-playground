import { expect, test } from "@playwright/test";

test("has title", async ({ page }) => {
	await page.goto("/");

	// Expect a title "to contain" a substring.
	await expect(page).toHaveTitle(/Leptos Tutorial/);
});

test("get started link", async ({ page }) => {
	await page.goto("/");

	await expect(page.getByRole("heading", { name: "Selector" })).toBeVisible();
});
