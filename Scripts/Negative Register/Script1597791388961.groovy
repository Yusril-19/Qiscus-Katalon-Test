import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://www.qiscus.com/multichannel/register')

WebUI.setText(findTestObject('Page_Qiscus Multichannel Chat/input_Company Name_company_name'), 'Negative Test')

WebUI.setText(findTestObject('Page_Qiscus Multichannel Chat/input_Work Email_work_email'), 'Negative Test')

WebUI.setText(findTestObject('Page_Qiscus Multichannel Chat/input_Phone Number_phone_number'), 'Negative Test')

WebUI.setEncryptedText(findTestObject('Page_Qiscus Multichannel Chat/input_Password_password'), 'tzH6RvlfSTg=')

WebUI.setEncryptedText(findTestObject('Page_Qiscus Multichannel Chat/input_Repeat Password_confirmation'), 'tzH6RvlfSTg=')

WebUI.click(findTestObject('Page_Qiscus Multichannel Chat/button_Register'))

WebUI.click(findTestObject('Object Repository/Page_Qiscus Multichannel Chat/small_The work_email field must be a valid email'))

WebUI.click(findTestObject('Object Repository/Page_Qiscus Multichannel Chat/small_The password field must be at least 6_e4d95f'))

WebUI.setText(findTestObject('Page_Qiscus Multichannel Chat/input_Work Email_work_email'), 'negative@gmail.com')

WebUI.setEncryptedText(findTestObject('Page_Qiscus Multichannel Chat/input_Password_password'), 'aeHFOx8jV/A=')

WebUI.setEncryptedText(findTestObject('Page_Qiscus Multichannel Chat/input_Repeat Password_confirmation'), 'CSt51UwWECc=')

WebUI.click(findTestObject('Object Repository/Page_Qiscus Multichannel Chat/small_The confirmation confirmation does not match'))

WebUI.closeBrowser()

